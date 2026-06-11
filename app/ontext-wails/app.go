package main

import (
	"context"
	"errors"
	"fmt"
	"os"

	"github.com/joho/godotenv"
	"github.com/wailsapp/wails/v2/pkg/runtime"

	"ontext-wails/internal/audio"
	"ontext-wails/internal/clipboard"
	"ontext-wails/internal/httpapi"
	"ontext-wails/internal/pipeline"
	"ontext-wails/internal/transcribe"
	"ontext-wails/internal/vad"
)

// debugAPIAddr is the local-only address for the Fiber debug/status server.
const debugAPIAddr = "127.0.0.1:34115"

// PasteResult mirrors the frontend's PasteResult type
// (frontend/src/types/transcript.ts).
type PasteResult struct {
	Success bool   `json:"success"`
	Error   string `json:"error,omitempty"`
}

// App struct
type App struct {
	ctx      context.Context
	pipeline *pipeline.Pipeline
}

// NewApp creates a new App application struct
func NewApp() *App {
	_ = godotenv.Load()

	var transcriber transcribe.Transcriber = transcribe.NoopTranscriber{}
	if apiKey := os.Getenv("GROQ"); apiKey != "" {
		transcriber = transcribe.NewGroqTranscriber(apiKey)
	}

	return &App{
		pipeline: &pipeline.Pipeline{
			Capturer:    audio.NewMalgoCapturer(),
			Detector:    vad.NewRMSDetector(),
			Transcriber: transcriber,
			Writer:      clipboard.NewWriter(),
		},
	}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx

	a.pipeline.OnStatus = func(status pipeline.Status) {
		runtime.EventsEmit(a.ctx, "status", string(status))
	}

	server := httpapi.New(a.pipeline)
	go server.Listen(debugAPIAddr)
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return fmt.Sprintf("Hello %s, It's show time!", name)
}

// StartPipeline runs one recording session (capture -> VAD -> transcribe ->
// paste) and returns once it completes.
func (a *App) StartPipeline() PasteResult {
	result := a.pipeline.Start(a.ctx)
	if result.Error != nil {
		return PasteResult{Success: false, Error: result.Error.Error()}
	}
	return PasteResult{Success: true}
}

// StopRecording ends the current recording session, if one is running.
func (a *App) StopRecording() error {
	return a.pipeline.Stop()
}

// RequestAccessibilityPermission requests macOS Accessibility permission.
// Native implementation lands in stage 12 (focus-paste); until then this
// always returns an error so the frontend falls back to opening System
// Settings directly.
func (a *App) RequestAccessibilityPermission() error {
	return errors.New("not implemented")
}
