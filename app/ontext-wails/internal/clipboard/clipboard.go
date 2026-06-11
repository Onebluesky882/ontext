// Package clipboard defines the output domain: delivering transcribed text
// to the user, either via the system clipboard or a focus-aware paste.
package clipboard

import "context"

// Writer delivers transcribed text to the active application.
type Writer interface {
	// Paste types or pastes text at the current cursor position in the
	// previously focused application.
	Paste(ctx context.Context, text string) error
}
