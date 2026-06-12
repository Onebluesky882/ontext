//go:build !darwin

package focus

// frontmostBundleID, activateBundleID and friends have no implementation
// outside macOS: focus tracking/reactivation is a no-op on Windows, where
// the standard paste keystroke already targets the focused window.

func frontmostBundleID() (string, error) { return "", nil }

func currentBundleID() string { return "" }

func activateBundleID(_ string) error { return nil }

func isAccessibilityTrusted() bool { return true }

func requestAccessibilityPermission() {}

func microphonePermissionStatus() MicrophonePermission { return MicrophoneAuthorized }

func requestMicrophonePermission() MicrophonePermission { return MicrophoneAuthorized }
