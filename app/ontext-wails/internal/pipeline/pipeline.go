// Package pipeline wires the audio, vad, transcribe and clipboard domains
// together into a single recording session, coordinated with channels and
// context cancellation.
package pipeline

import (
	"context"
	"errors"
	"fmt"
	"sync"

	"ontext-wails/internal/audio"
	"ontext-wails/internal/clipboard"
	"ontext-wails/internal/transcribe"
	"ontext-wails/internal/vad"
)

// Status reports pipeline lifecycle changes for the UI layer (emitted via
// Wails runtime events by the caller).
type Status string

const (
	StatusIdle    Status = "idle"
	StatusRunning Status = "running"
	StatusDone    Status = "done"
	StatusError   Status = "error"
)

// Result is the final outcome of a recording session.
type Result struct {
	Text  string
	Error error
}

// Pipeline runs one recording session: capture -> VAD -> transcribe -> paste.
type Pipeline struct {
	Capturer    audio.Capturer
	Detector    vad.Detector
	Transcriber transcribe.Transcriber
	Writer      clipboard.Writer

	OnStatus func(Status)

	mu     sync.Mutex
	cancel context.CancelFunc
}

// Start begins capturing audio and processing it through the pipeline. It
// returns once the session completes (microphone stopped, or ctx canceled),
// delivering exactly one Result.
func (p *Pipeline) Start(ctx context.Context) Result {
	p.mu.Lock()
	if p.cancel != nil {
		p.mu.Unlock()
		return Result{Error: errors.New("pipeline already running")}
	}
	ctx, cancel := context.WithCancel(ctx)
	p.cancel = cancel
	p.mu.Unlock()

	defer func() {
		p.mu.Lock()
		p.cancel = nil
		p.mu.Unlock()
	}()

	p.setStatus(StatusRunning)

	frames, err := p.Capturer.Start(ctx)
	if err != nil {
		p.setStatus(StatusError)
		return Result{Error: fmt.Errorf("start capture: %w", err)}
	}
	defer p.Capturer.Stop()

	segments := p.Detector.Detect(ctx, frames)

	var text string
	for segment := range segments {
		res, err := p.Transcriber.Transcribe(ctx, segment)
		if err != nil {
			p.setStatus(StatusError)
			return Result{Error: fmt.Errorf("transcribe: %w", err)}
		}
		text += res.Text
	}

	if text != "" {
		if err := p.Writer.Paste(ctx, text); err != nil {
			p.setStatus(StatusError)
			return Result{Error: fmt.Errorf("paste: %w", err)}
		}
	}

	p.setStatus(StatusDone)
	return Result{Text: text}
}

// Stop ends the current recording session, if one is running.
func (p *Pipeline) Stop() error {
	p.mu.Lock()
	defer p.mu.Unlock()
	if p.cancel == nil {
		return errors.New("pipeline not running")
	}
	p.cancel()
	return nil
}

func (p *Pipeline) setStatus(s Status) {
	if p.OnStatus != nil {
		p.OnStatus(s)
	}
}
