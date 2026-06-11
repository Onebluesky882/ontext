// Package transcribe defines the transcription domain: turning a speech
// Segment into text via a transcription backend (e.g. Groq Whisper).
package transcribe

import (
	"context"

	"ontext-wails/internal/vad"
)

// Result is the outcome of transcribing a single speech segment.
type Result struct {
	Text string

	// Language is the detected language code (e.g. "th").
	Language string

	// Confidence diagnostics from the Whisper API, used to filter
	// likely hallucinations on silent/non-speech audio.
	NoSpeechProb     float32
	AvgLogprob       float32
	CompressionRatio float32
}

// Hallucination thresholds, mirroring modules/transcribe (Rust).
const (
	NoSpeechProbThreshold     = 0.5
	AvgLogprobThreshold       = -1.0
	CompressionRatioThreshold = 2.4
)

// IsLikelyHallucination reports whether the result's confidence diagnostics
// indicate the transcript is probably a hallucination on non-speech audio.
func (r Result) IsLikelyHallucination() bool {
	return r.NoSpeechProb > NoSpeechProbThreshold ||
		r.AvgLogprob < AvgLogprobThreshold ||
		r.CompressionRatio > CompressionRatioThreshold
}

// Transcriber converts a speech segment into text.
type Transcriber interface {
	Transcribe(ctx context.Context, segment vad.Segment) (Result, error)
}
