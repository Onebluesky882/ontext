package transcribe

import (
	"context"

	"ontext-wails/internal/vad"
)

// NoopTranscriber returns empty results. It's a placeholder until the Groq
// Whisper HTTP client is implemented.
type NoopTranscriber struct{}

func (NoopTranscriber) Transcribe(ctx context.Context, segment vad.Segment) (Result, error) {
	return Result{}, nil
}
