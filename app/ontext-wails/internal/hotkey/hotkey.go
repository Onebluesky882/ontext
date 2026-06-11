// Package hotkey provides a global hold-to-talk hotkey listener: pressing
// the hotkey starts the recording pipeline, releasing it stops the
// pipeline, and the hold's start/end timestamps are made available for
// usage-session reporting (ADR 010), with graceful degradation if the
// hotkey cannot be registered (e.g. missing OS permission or a conflicting
// binding).
package hotkey

// Listener registers a global hotkey and reports key-down/key-up events.
// Implementations must not block in Register or Unregister.
type Listener interface {
	// Register starts listening for the hotkey. It returns an error if the
	// hotkey could not be registered; callers should fall back to
	// button-only operation in that case.
	Register() error

	// KeyDown returns a channel that receives a value each time the hotkey
	// is pressed. It is only valid after a successful Register call.
	KeyDown() <-chan struct{}

	// KeyUp returns a channel that receives a value each time the hotkey
	// is released. It is only valid after a successful Register call.
	KeyUp() <-chan struct{}

	// Unregister stops listening and releases the hotkey.
	Unregister()
}
