package vad

import (
	"context"

	"ontext-wails/internal/audio"
)

// NoopDetector passes no segments through. It's a placeholder until a real
// VAD implementation (e.g. RMS-based) is ported from modules/vad.
type NoopDetector struct{}

func (NoopDetector) Detect(ctx context.Context, frames <-chan audio.Frame) <-chan Segment {
	out := make(chan Segment)
	go func() {
		defer close(out)
		for range frames {
		}
	}()
	return out
}
