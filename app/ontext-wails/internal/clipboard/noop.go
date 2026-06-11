package clipboard

import "context"

// NoopWriter discards text instead of pasting it. It's a placeholder until
// the focus-aware paste adapter (e.g. robotgo) is implemented.
type NoopWriter struct{}

func (NoopWriter) Paste(ctx context.Context, text string) error { return nil }
