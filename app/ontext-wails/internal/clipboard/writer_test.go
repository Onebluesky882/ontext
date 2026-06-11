package clipboard_test

import (
	"context"
	"os"
	"testing"

	"ontext-wails/internal/clipboard"
)

// Compile-time check: ClipboardWriter must satisfy Writer.
var _ clipboard.Writer = (*clipboard.ClipboardWriter)(nil)

func TestNewWriter_ReturnsNonNil(t *testing.T) {
	w := clipboard.NewWriter()
	if w == nil {
		t.Fatal("NewWriter() returned nil")
	}
}

// TestPaste_Integration runs a real paste cycle. It is skipped unless the
// CLIPBOARD_TEST environment variable is set (display + clipboard required).
func TestPaste_Integration(t *testing.T) {
	if os.Getenv("CLIPBOARD_TEST") == "" {
		t.Skip("CLIPBOARD_TEST not set; skipping integration test (requires display + clipboard)")
	}

	w := clipboard.NewWriter()
	if err := w.Paste(context.Background(), "ontext clipboard test"); err != nil {
		t.Fatalf("Paste() returned unexpected error: %v", err)
	}
}

// TestNoopWriter_Satisfies_Writer ensures NoopWriter still satisfies the interface.
var _ clipboard.Writer = clipboard.NoopWriter{}

func TestNoopWriter_Paste_ReturnsNil(t *testing.T) {
	w := clipboard.NoopWriter{}
	if err := w.Paste(context.Background(), "hello"); err != nil {
		t.Fatalf("NoopWriter.Paste() returned unexpected error: %v", err)
	}
}
