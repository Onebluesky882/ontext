package transcribe

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"mime/multipart"
	"net/http"
	"time"

	"ontext-wails/internal/vad"
)

const (
	defaultBaseURL = "https://api.groq.com/openai"
	defaultModel   = "whisper-large-v3"
	defaultTimeout = 30 * time.Second
)

// GroqTranscriber transcribes speech segments via the Groq Whisper API.
type GroqTranscriber struct {
	APIKey  string
	BaseURL string
	Model   string
	Client  *http.Client
}

// NewGroqTranscriber creates a transcriber using the given API key and the
// default Groq endpoint and whisper-large-v3 model.
func NewGroqTranscriber(apiKey string) *GroqTranscriber {
	return &GroqTranscriber{
		APIKey:  apiKey,
		BaseURL: defaultBaseURL,
		Model:   defaultModel,
		Client:  &http.Client{Timeout: defaultTimeout},
	}
}

type whisperSegment struct {
	AvgLogprob       float32 `json:"avg_logprob"`
	NoSpeechProb     float32 `json:"no_speech_prob"`
	CompressionRatio float32 `json:"compression_ratio"`
}

type whisperVerboseResponse struct {
	Text     string           `json:"text"`
	Language string           `json:"language"`
	Segments []whisperSegment `json:"segments"`
}

// Transcribe sends the segment's audio to the Groq Whisper API and returns
// the transcription along with hallucination-detection diagnostics.
func (g *GroqTranscriber) Transcribe(ctx context.Context, segment vad.Segment) (Result, error) {
	if g.APIKey == "" {
		return Result{}, errors.New("groq: API key is empty")
	}

	wavBytes := encodeWAV(segment.Samples, segment.SampleRate)

	var body bytes.Buffer
	writer := multipart.NewWriter(&body)

	part, err := writer.CreateFormFile("file", "audio.wav")
	if err != nil {
		return Result{}, fmt.Errorf("groq: create form file: %w", err)
	}
	if _, err := part.Write(wavBytes); err != nil {
		return Result{}, fmt.Errorf("groq: write audio: %w", err)
	}

	model := g.Model
	if model == "" {
		model = defaultModel
	}
	_ = writer.WriteField("model", model)
	_ = writer.WriteField("response_format", "verbose_json")
	_ = writer.WriteField("language", "th")

	if err := writer.Close(); err != nil {
		return Result{}, fmt.Errorf("groq: close multipart writer: %w", err)
	}

	url := g.BaseURL + "/v1/audio/transcriptions"
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, url, &body)
	if err != nil {
		return Result{}, fmt.Errorf("groq: build request: %w", err)
	}
	req.Header.Set("Content-Type", writer.FormDataContentType())
	req.Header.Set("Authorization", "Bearer "+g.APIKey)

	resp, err := g.Client.Do(req)
	if err != nil {
		return Result{}, fmt.Errorf("groq: request failed: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		var errBody bytes.Buffer
		errBody.ReadFrom(resp.Body)
		return Result{}, fmt.Errorf("groq: API error %d: %s", resp.StatusCode, errBody.String())
	}

	var parsed whisperVerboseResponse
	if err := json.NewDecoder(resp.Body).Decode(&parsed); err != nil {
		return Result{}, fmt.Errorf("groq: decode response: %w", err)
	}

	result := Result{
		Text:     parsed.Text,
		Language: parsed.Language,
	}
	if len(parsed.Segments) > 0 {
		var noSpeechSum, logprobSum, compressionSum float32
		for _, s := range parsed.Segments {
			noSpeechSum += s.NoSpeechProb
			logprobSum += s.AvgLogprob
			compressionSum += s.CompressionRatio
		}
		count := float32(len(parsed.Segments))
		result.NoSpeechProb = noSpeechSum / count
		result.AvgLogprob = logprobSum / count
		result.CompressionRatio = compressionSum / count
	}

	return result, nil
}
