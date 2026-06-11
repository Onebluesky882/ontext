// Package vad defines the voice-activity-detection domain: it turns a
// stream of raw audio frames into discrete speech Segments.
package vad

import (
	"context"

	"ontext-wails/internal/audio"
)

// Segment is one continuous span of detected speech, ready for transcription.
type Segment struct {
	Samples    []float32
	SampleRate int
}

// Detector consumes audio frames and emits speech segments. It must close
// the returned channel once frames is closed or ctx is canceled.
type Detector interface {
	Detect(ctx context.Context, frames <-chan audio.Frame) <-chan Segment
}
