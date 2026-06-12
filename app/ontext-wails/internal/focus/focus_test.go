package focus

import "testing"

func TestMicrophonePermissionString(t *testing.T) {
	cases := map[MicrophonePermission]string{
		MicrophoneNotDetermined: "not_determined",
		MicrophoneRestricted:    "restricted",
		MicrophoneDenied:        "denied",
		MicrophoneAuthorized:    "authorized",
	}

	for perm, want := range cases {
		if got := perm.String(); got != want {
			t.Errorf("MicrophonePermission(%d).String() = %q, want %q", perm, got, want)
		}
	}
}

func TestMicrophonePermissionStatusDoesNotPanic(t *testing.T) {
	// MicrophonePermissionStatus must not crash regardless of platform or
	// permission state.
	_ = MicrophonePermissionStatus()
}
