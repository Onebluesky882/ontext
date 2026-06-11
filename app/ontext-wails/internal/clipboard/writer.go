package clipboard

import (
	"context"
	"fmt"
	"runtime"

	"github.com/atotto/clipboard"
	"github.com/go-vgo/robotgo"
)

// ClipboardWriter writes text to the system clipboard and simulates a paste
// keystroke (Cmd+V on macOS, Ctrl+V on Windows) into the active application.
type ClipboardWriter struct{}

// NewWriter returns a ClipboardWriter backed by atotto/clipboard and robotgo.
func NewWriter() *ClipboardWriter {
	return &ClipboardWriter{}
}

// Paste writes text to the system clipboard, then simulates the OS paste shortcut
// to deliver the text to the currently focused application.
func (w *ClipboardWriter) Paste(_ context.Context, text string) error {
	if err := clipboard.WriteAll(text); err != nil {
		return fmt.Errorf("clipboard write failed: %w", err)
	}
	return simulatePaste()
}

// Clear selects all content in the focused field and deletes it (Cmd+A,
// Backspace on macOS; Ctrl+A, Backspace on Windows). Used by debug/testing
// flows so each session starts from an empty field.
func (w *ClipboardWriter) Clear(_ context.Context) error {
	return simulateClear()
}

func simulatePaste() error {
	switch runtime.GOOS {
	case "darwin":
		var tapErr error
		runOnMainThread(func() {
			tapErr = robotgo.KeyTap("v", "command")
		})
		if tapErr != nil {
			return fmt.Errorf("paste simulation failed: %w", tapErr)
		}
	case "windows":
		if err := robotgo.KeyTap("v", "ctrl"); err != nil {
			return fmt.Errorf("paste simulation failed: %w", err)
		}
	default:
		return fmt.Errorf("paste simulation: unsupported platform %q", runtime.GOOS)
	}
	return nil
}

func simulateClear() error {
	switch runtime.GOOS {
	case "darwin":
		var err error
		runOnMainThread(func() {
			if e := robotgo.KeyTap("a", "command"); e != nil {
				err = e
				return
			}
			err = robotgo.KeyTap("backspace")
		})
		if err != nil {
			return fmt.Errorf("clear simulation failed: %w", err)
		}
	case "windows":
		if err := robotgo.KeyTap("a", "ctrl"); err != nil {
			return fmt.Errorf("clear simulation failed: %w", err)
		}
		if err := robotgo.KeyTap("backspace"); err != nil {
			return fmt.Errorf("clear simulation failed: %w", err)
		}
	default:
		return fmt.Errorf("clear simulation: unsupported platform %q", runtime.GOOS)
	}
	return nil
}
