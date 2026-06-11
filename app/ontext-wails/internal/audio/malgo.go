package audio

import (
	"context"
	"encoding/binary"
	"fmt"
	"math"
	"sync"

	"github.com/gen2brain/malgo"
)

// MalgoCapturer streams 16 kHz mono f32 frames from the default microphone
// using the miniaudio library (via gen2brain/malgo).
type MalgoCapturer struct {
	mu     sync.Mutex
	device *malgo.Device
	mctx   *malgo.AllocatedContext
	cancel context.CancelFunc
}

func NewMalgoCapturer() *MalgoCapturer {
	return &MalgoCapturer{}
}

func (c *MalgoCapturer) Start(ctx context.Context) (<-chan Frame, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	if c.device != nil {
		return nil, fmt.Errorf("audio: already capturing")
	}

	mctx, err := malgo.InitContext(nil, malgo.ContextConfig{}, func(string) {})
	if err != nil {
		return nil, fmt.Errorf("audio: init context: %w", err)
	}

	ch := make(chan Frame, 64)
	captureCtx, cancel := context.WithCancel(ctx)

	cfg := malgo.DefaultDeviceConfig(malgo.Capture)
	cfg.Capture.Format = malgo.FormatF32
	cfg.Capture.Channels = 1
	cfg.SampleRate = 16000

	callbacks := malgo.DeviceCallbacks{
		Data: func(_, input []byte, frameCount uint32) {
			samples := decodePCMF32(input, frameCount)
			select {
			case ch <- Frame{Samples: samples, SampleRate: 16000}:
			case <-captureCtx.Done():
			}
		},
	}

	device, err := malgo.InitDevice(mctx.Context, cfg, callbacks)
	if err != nil {
		cancel()
		mctx.Uninit()
		return nil, fmt.Errorf("audio: init device: %w", err)
	}

	if err := device.Start(); err != nil {
		cancel()
		device.Uninit()
		mctx.Uninit()
		return nil, fmt.Errorf("audio: start device: %w", err)
	}

	c.device = device
	c.mctx = mctx
	c.cancel = cancel

	go func() {
		<-captureCtx.Done()
		device.Stop()
		device.Uninit()
		mctx.Uninit()
		c.mu.Lock()
		c.device = nil
		c.mctx = nil
		c.mu.Unlock()
		close(ch)
	}()

	return ch, nil
}

func (c *MalgoCapturer) Stop() error {
	c.mu.Lock()
	cancel := c.cancel
	c.cancel = nil
	c.mu.Unlock()
	if cancel != nil {
		cancel()
	}
	return nil
}

// decodePCMF32 converts raw little-endian f32 PCM bytes to a float32 slice.
func decodePCMF32(b []byte, frameCount uint32) []float32 {
	samples := make([]float32, frameCount)
	for i := uint32(0); i < frameCount && int((i+1)*4) <= len(b); i++ {
		bits := binary.LittleEndian.Uint32(b[i*4:])
		samples[i] = math.Float32frombits(bits)
	}
	return samples
}
