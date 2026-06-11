//go:build !darwin

package clipboard

// runOnMainThread runs fn directly. Only macOS requires paste simulation to
// run on the main thread (Carbon/TSM keyboard-layout APIs).
func runOnMainThread(fn func()) {
	fn()
}
