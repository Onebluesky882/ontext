package main

import (
	"context"
	"fmt"
	"os"

	"github.com/joho/godotenv"

	"ontext-wails/internal/audio"
	"ontext-wails/internal/clipboard"
	"ontext-wails/internal/httpapi"
	"ontext-wails/internal/pipeline"
	"ontext-wails/internal/transcribe"
	"ontext-wails/internal/vad"
)

// debugAPIAddr is the local-only address for the Fiber debug/status server.
const debugAPIAddr = "127.0.0.1:34115"

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
			Capturer:    audio.NoopCapturer{},
			Detector:    vad.NoopDetector{},
			Transcriber: transcriber,
			Writer:      clipboard.NoopWriter{},
		},
	}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx

	server := httpapi.New(a.pipeline)
	go server.Listen(debugAPIAddr)
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) string {
	return fmt.Sprintf("Hello %s, It's show time!", name)
}
