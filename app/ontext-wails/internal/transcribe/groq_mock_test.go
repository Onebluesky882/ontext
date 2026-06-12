package transcribe

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"ontext-wails/internal/vad"
)

func makeSegment(samples []float32) vad.Segment {
	return vad.Segment{Samples: samples, SampleRate: 16000}
}

func newTestTranscriber(baseURL string) *GroqTranscriber {
	return &GroqTranscriber{
		APIKey:  "test-key",
		BaseURL: baseURL,
		Model:   defaultModel,
		Client:  &http.Client{Timeout: 5 * time.Second},
	}
}

func TestGroqTranscriber_EmptyAPIKey(t *testing.T) {
	tr := &GroqTranscriber{APIKey: ""}
	_, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err == nil {
		t.Fatal("expected error for empty API key, got nil")
	}
}

func TestGroqTranscriber_SuccessEnglish(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"text":"  Hello world  ","language":"english","segments":[]}`))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	result, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0, 0.1, -0.1}))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Text != "Hello world" {
		t.Errorf("expected trimmed %q, got %q", "Hello world", result.Text)
	}
	if result.Language != "english" {
		t.Errorf("expected language %q, got %q", "english", result.Language)
	}
}

func TestGroqTranscriber_SuccessThai(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"text":" สวัสดี ","language":"thai","segments":[]}`))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	result, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Text != "สวัสดี" {
		t.Errorf("expected %q, got %q", "สวัสดี", result.Text)
	}
}

func TestGroqTranscriber_TextIsTrimmed(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"text":"\n  trimmed text \t\n","language":"english","segments":[]}`))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	result, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result.Text != "trimmed text" {
		t.Errorf("expected %q, got %q", "trimmed text", result.Text)
	}
}

func TestGroqTranscriber_APIError401(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		w.Write([]byte(`{"error":{"message":"Invalid API key"}}`))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	_, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err == nil {
		t.Fatal("expected error for 401, got nil")
	}
}

func TestGroqTranscriber_ServerError500(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("Internal Server Error"))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	_, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err == nil {
		t.Fatal("expected error for 500, got nil")
	}
}

func TestGroqTranscriber_ContextTimeout(t *testing.T) {
	unblock := make(chan struct{})
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		select {
		case <-unblock:
		case <-time.After(5 * time.Second):
		}
	}))
	defer func() {
		close(unblock)
		srv.Close()
	}()

	tr := newTestTranscriber(srv.URL)
	ctx, cancel := context.WithTimeout(context.Background(), 50*time.Millisecond)
	defer cancel()

	_, err := tr.Transcribe(ctx, makeSegment([]float32{0.0}))
	if err == nil {
		t.Fatal("expected error on timeout, got nil")
	}
}

func TestGroqTranscriber_HallucinationDiagnostics(t *testing.T) {
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{
			"text": "test",
			"language": "english",
			"segments": [
				{"avg_logprob": -0.5, "no_speech_prob": 0.8, "compression_ratio": 1.2},
				{"avg_logprob": -0.3, "no_speech_prob": 0.2, "compression_ratio": 1.8}
			]
		}`))
	}))
	defer srv.Close()

	tr := newTestTranscriber(srv.URL)
	result, err := tr.Transcribe(context.Background(), makeSegment([]float32{0.0}))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	wantNoSpeech := float32((0.8 + 0.2) / 2)
	wantLogprob := float32((-0.5 + -0.3) / 2)
	wantCompression := float32((1.2 + 1.8) / 2)

	if result.NoSpeechProb != wantNoSpeech {
		t.Errorf("NoSpeechProb: got %.3f, want %.3f", result.NoSpeechProb, wantNoSpeech)
	}
	if result.AvgLogprob != wantLogprob {
		t.Errorf("AvgLogprob: got %.3f, want %.3f", result.AvgLogprob, wantLogprob)
	}
	if result.CompressionRatio != wantCompression {
		t.Errorf("CompressionRatio: got %.3f, want %.3f", result.CompressionRatio, wantCompression)
	}
}

func TestIsLikelyHallucination(t *testing.T) {
	cases := []struct {
		name   string
		result Result
		want   bool
	}{
		{"clean result", Result{NoSpeechProb: 0.1, AvgLogprob: -0.5, CompressionRatio: 1.5}, false},
		{"high no_speech_prob", Result{NoSpeechProb: 0.6, AvgLogprob: -0.5, CompressionRatio: 1.5}, true},
		{"low avg_logprob", Result{NoSpeechProb: 0.1, AvgLogprob: -1.5, CompressionRatio: 1.5}, true},
		{"high compression_ratio", Result{NoSpeechProb: 0.1, AvgLogprob: -0.5, CompressionRatio: 3.0}, true},
		{"repeated char text", Result{Text: "vvvvvvvvvvvv", NoSpeechProb: 0.1, AvgLogprob: -0.5, CompressionRatio: 1.5}, true},
		{"short repeated char text", Result{Text: "vv", NoSpeechProb: 0.1, AvgLogprob: -0.5, CompressionRatio: 1.5}, false},
		{"normal text not flagged", Result{Text: "สวัสดีครับ", NoSpeechProb: 0.1, AvgLogprob: -0.5, CompressionRatio: 1.5}, false},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got := tc.result.IsLikelyHallucination()
			if got != tc.want {
				t.Errorf("IsLikelyHallucination() = %v, want %v", got, tc.want)
			}
		})
	}
}
