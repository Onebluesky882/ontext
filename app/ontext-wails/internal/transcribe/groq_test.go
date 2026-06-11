package transcribe

import (
	"context"
	"os"
	"testing"

	"github.com/joho/godotenv"

	"ontext-wails/internal/vad"
)

// TestGroqTranscriber_RealAPI hits the live Groq Whisper API using the GROQ
// key from .env. It's skipped if GROQ isn't set (e.g. in CI).
func TestGroqTranscriber_RealAPI(t *testing.T) {
	_ = godotenv.Load("../../.env")

	apiKey := os.Getenv("GROQ")
	if apiKey == "" {
		t.Skip("GROQ env var not set; skipping live API test")
	}

	const sampleRate = 16000
	samples := make([]float32, sampleRate) // 1s of silence

	tr := NewGroqTranscriber(apiKey)
	if tr.Model != "whisper-large-v3" {
		t.Fatalf("expected default model whisper-large-v3, got %q", tr.Model)
	}

	result, err := tr.Transcribe(context.Background(), vad.Segment{
		Samples:    samples,
		SampleRate: sampleRate,
	})
	if err != nil {
		t.Fatalf("Transcribe: %v", err)
	}

	t.Logf("text=%q language=%q no_speech_prob=%.3f avg_logprob=%.3f compression_ratio=%.3f hallucination=%v",
		result.Text, result.Language, result.NoSpeechProb, result.AvgLogprob, result.CompressionRatio,
		result.IsLikelyHallucination())
}
