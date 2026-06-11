package vad

import (
	"context"
	"math"
	"testing"
	"time"

	"ontext-wails/internal/audio"
)

// makeSilence returns n samples of near-zero amplitude (below threshold).
func makeSilence(n int) []float32 {
	return make([]float32, n)
}

// makeTone returns a sine wave at freq Hz sampled at sampleRate for durationMs.
// Amplitude 0.8 — well above the 0.02 RMS threshold.
func makeTone(freqHz float64, sampleRate int, durationMs int) []float32 {
	n := sampleRate * durationMs / 1000
	out := make([]float32, n)
	for i := range out {
		t := float64(i) / float64(sampleRate)
		out[i] = float32(math.Sin(2*math.Pi*freqHz*t) * 0.8)
	}
	return out
}

// sendFrames pushes samples as single-frame messages into a channel, then closes it.
func sendFrames(ch chan<- audio.Frame, samples []float32, sampleRate int) {
	const frameSize = 480 // 30 ms at 16 kHz
	for i := 0; i < len(samples); i += frameSize {
		end := i + frameSize
		if end > len(samples) {
			end = len(samples)
		}
		chunk := make([]float32, end-i)
		copy(chunk, samples[i:end])
		ch <- audio.Frame{Samples: chunk, SampleRate: sampleRate}
	}
	close(ch)
}

func collectSegments(out <-chan Segment) []Segment {
	var segs []Segment
	for s := range out {
		segs = append(segs, s)
	}
	return segs
}

// --- NoopDetector ---

func TestNoopDetector_ClosedInput(t *testing.T) {
	ch := make(chan audio.Frame)
	close(ch)
	out := NoopDetector{}.Detect(context.Background(), ch)
	segs := collectSegments(out)
	if len(segs) != 0 {
		t.Fatalf("NoopDetector: expected no segments, got %d", len(segs))
	}
}

// --- RMSDetector ---

func TestRMSDetector_EmptyInput(t *testing.T) {
	ch := make(chan audio.Frame)
	close(ch)
	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)
	if len(segs) != 0 {
		t.Fatalf("expected no segments for empty input, got %d", len(segs))
	}
}

func TestRMSDetector_SilenceOnly(t *testing.T) {
	ch := make(chan audio.Frame, 64)
	go sendFrames(ch, makeSilence(16000), 16000) // 1 second silence

	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)
	if len(segs) != 0 {
		t.Fatalf("silence-only input should produce no segments, got %d", len(segs))
	}
}

func TestRMSDetector_SpeechPreserved(t *testing.T) {
	// 300ms silence + 600ms tone + 300ms silence
	samples := makeSilence(16000 * 300 / 1000)
	samples = append(samples, makeTone(400, 16000, 600)...)
	samples = append(samples, makeSilence(16000*300/1000)...)

	ch := make(chan audio.Frame, 128)
	go sendFrames(ch, samples, 16000)

	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)

	if len(segs) == 0 {
		t.Fatal("expected at least one speech segment, got none")
	}
	for _, s := range segs {
		if len(s.Samples) == 0 {
			t.Error("segment has zero samples")
		}
		if s.SampleRate != 16000 {
			t.Errorf("expected SampleRate 16000, got %d", s.SampleRate)
		}
	}
}

func TestRMSDetector_ShortSpeechDiscarded(t *testing.T) {
	// 100ms tone — shorter than minChunkMs (500ms), should be discarded
	samples := makeTone(400, 16000, 100)

	ch := make(chan audio.Frame, 64)
	go sendFrames(ch, samples, 16000)

	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)

	if len(segs) != 0 {
		t.Fatalf("short speech below minChunkMs should be discarded, got %d segment(s)", len(segs))
	}
}

func TestRMSDetector_LongSpeechFlushed(t *testing.T) {
	// 9 seconds of tone — exceeds maxChunkMs (8s), should produce ≥2 segments
	samples := makeTone(400, 16000, 9000)

	ch := make(chan audio.Frame, 512)
	go sendFrames(ch, samples, 16000)

	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)

	if len(segs) < 2 {
		t.Fatalf("9s speech should produce ≥2 segments (maxChunkMs=8s), got %d", len(segs))
	}
}

func TestRMSDetector_ContextCancel(t *testing.T) {
	// Infinite silence stream — cancel ctx to unblock
	ch := make(chan audio.Frame)
	ctx, cancel := context.WithCancel(context.Background())

	det := NewRMSDetector()
	out := det.Detect(ctx, ch)

	cancel()

	done := make(chan struct{})
	go func() {
		for range out {
		}
		close(done)
	}()

	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("Detect goroutine did not exit after ctx cancel")
	}
}

func TestRMSDetector_MultipleSegments(t *testing.T) {
	// silence – speech – silence – speech – silence
	sr := 16000
	silence := func(ms int) []float32 { return makeSilence(sr * ms / 1000) }
	speech := func(ms int) []float32 { return makeTone(400, sr, ms) }

	samples := silence(200)
	samples = append(samples, speech(600)...)
	samples = append(samples, silence(1500)...) // 1.5s silence → segment boundary
	samples = append(samples, speech(600)...)
	samples = append(samples, silence(200)...)

	ch := make(chan audio.Frame, 256)
	go sendFrames(ch, samples, sr)

	det := NewRMSDetector()
	out := det.Detect(context.Background(), ch)
	segs := collectSegments(out)

	if len(segs) < 2 {
		t.Fatalf("expected at least 2 segments from two speech bursts, got %d", len(segs))
	}
}
