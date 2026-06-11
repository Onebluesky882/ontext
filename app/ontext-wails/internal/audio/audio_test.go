package audio

import (
	"context"
	"math"
	"testing"
	"time"
)

// compile-time interface checks
var _ Capturer = NoopCapturer{}
var _ Capturer = (*MalgoCapturer)(nil)

func TestNoopCapturer_ClosesImmediately(t *testing.T) {
	c := NoopCapturer{}
	ch, err := c.Start(context.Background())
	if err != nil {
		t.Fatalf("Start: %v", err)
	}
	select {
	case _, ok := <-ch:
		if ok {
			t.Fatal("expected channel to be closed, got a frame")
		}
	case <-time.After(time.Second):
		t.Fatal("channel not closed within 1s")
	}
	if err := c.Stop(); err != nil {
		t.Fatalf("Stop: %v", err)
	}
}

func TestMalgoCapturer_StopBeforeStart(t *testing.T) {
	c := NewMalgoCapturer()
	if err := c.Stop(); err != nil {
		t.Fatalf("Stop before Start should return nil, got: %v", err)
	}
}

func TestMalgoCapturer_DoubleStart(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping device test in short mode")
	}

	c := NewMalgoCapturer()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	ch, err := c.Start(ctx)
	if err != nil {
		t.Skipf("no audio device available: %v", err)
	}
	defer func() { _ = c.Stop(); drain(ch) }()

	_, err = c.Start(ctx)
	if err == nil {
		t.Fatal("second Start should return an error")
	}
}

func TestMalgoCapturer_StartStop(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping device test in short mode")
	}

	c := NewMalgoCapturer()
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	ch, err := c.Start(ctx)
	if err != nil {
		t.Skipf("no audio device available: %v", err)
	}

	// Collect a few frames then stop.
	var frames []Frame
	for len(frames) < 3 {
		select {
		case f, ok := <-ch:
			if !ok {
				t.Fatal("channel closed before Stop")
			}
			frames = append(frames, f)
		case <-ctx.Done():
			t.Fatal("timeout waiting for frames")
		}
	}

	if err := c.Stop(); err != nil {
		t.Fatalf("Stop: %v", err)
	}

	// Drain until closed.
	drain(ch)

	for i, f := range frames {
		if f.SampleRate != 16000 {
			t.Errorf("frame %d: SampleRate = %d, want 16000", i, f.SampleRate)
		}
		if len(f.Samples) == 0 {
			t.Errorf("frame %d: empty Samples", i)
		}
		for _, s := range f.Samples {
			if math.IsNaN(float64(s)) || math.IsInf(float64(s), 0) {
				t.Errorf("frame %d: invalid sample value %v", i, s)
			}
		}
	}
}

func TestDecodePCMF32(t *testing.T) {
	// Encode 1.0 and -0.5 as little-endian f32.
	cases := []float32{1.0, -0.5, 0.0}
	buf := make([]byte, len(cases)*4)
	for i, v := range cases {
		bits := math.Float32bits(v)
		buf[i*4] = byte(bits)
		buf[i*4+1] = byte(bits >> 8)
		buf[i*4+2] = byte(bits >> 16)
		buf[i*4+3] = byte(bits >> 24)
	}

	got := decodePCMF32(buf, uint32(len(cases)))
	if len(got) != len(cases) {
		t.Fatalf("len = %d, want %d", len(got), len(cases))
	}
	for i, want := range cases {
		if got[i] != want {
			t.Errorf("[%d] got %v, want %v", i, got[i], want)
		}
	}
}

func drain(ch <-chan Frame) {
	for range ch {
	}
}
