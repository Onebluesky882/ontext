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

func simulatePaste() error {
	switch runtime.GOOS {
	case "darwin":
		if err := robotgo.KeyTap("v", "command"); err != nil {
			return fmt.Errorf("paste simulation failed: %w", err)
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
