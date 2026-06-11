// Package audio defines the audio capture domain: the Frame type produced
// by a microphone source and the Capturer port that pipeline depends on.
package audio

import "context"

// Frame is a chunk of mono PCM audio samples.
type Frame struct {
	Samples    []float32
	SampleRate int
}

// Capturer streams audio frames from an input device until ctx is canceled
// or Stop is called. The returned channel is closed when capture ends.
type Capturer interface {
	Start(ctx context.Context) (<-chan Frame, error)
	Stop() error
}
