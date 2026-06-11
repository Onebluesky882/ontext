package audio

import "context"

// NoopCapturer produces no audio frames. It's a placeholder until a real
// device-backed Capturer (e.g. malgo) is implemented.
type NoopCapturer struct{}

func (NoopCapturer) Start(ctx context.Context) (<-chan Frame, error) {
	ch := make(chan Frame)
	close(ch)
	return ch, nil
}

func (NoopCapturer) Stop() error { return nil }
