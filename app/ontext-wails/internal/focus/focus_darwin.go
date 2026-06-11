//go:build darwin

package focus

/*
#cgo CFLAGS: -x objective-c -fobjc-arc
#cgo LDFLAGS: -framework AppKit -framework ApplicationServices
#import <AppKit/AppKit.h>
#import <ApplicationServices/ApplicationServices.h>
#include <stdlib.h>

static char *focus_frontmost_bundle_id(void) {
    NSRunningApplication *app = [[NSWorkspace sharedWorkspace] frontmostApplication];
    if (app == nil) {
        return NULL;
    }
    NSString *bid = [app bundleIdentifier];
    if (bid == nil) {
        return NULL;
    }
    return strdup([bid UTF8String]);
}

static char *focus_current_bundle_id(void) {
    NSString *bid = [[NSBundle mainBundle] bundleIdentifier];
    if (bid == nil) {
        return NULL;
    }
    return strdup([bid UTF8String]);
}

static int focus_activate_bundle_id(const char *bundleID) {
    NSString *bid = [NSString stringWithUTF8String:bundleID];
    NSArray<NSRunningApplication *> *apps =
        [NSRunningApplication runningApplicationsWithBundleIdentifier:bid];
    if (apps.count == 0) {
        return 0;
    }
    return [apps[0] activateWithOptions:NSApplicationActivateAllWindows] ? 1 : 0;
}

static int focus_is_accessibility_trusted(void) {
    return AXIsProcessTrusted() ? 1 : 0;
}

static void focus_request_accessibility_permission(void) {
    NSDictionary *options = @{(__bridge id)kAXTrustedCheckOptionPrompt: @YES};
    AXIsProcessTrustedWithOptions((CFDictionaryRef)options);
}
*/
import "C"

import (
	"errors"
	"unsafe"
)

// frontmostBundleID returns the bundle id of the current frontmost
// application, or "" if it cannot be determined.
func frontmostBundleID() (string, error) {
	cstr := C.focus_frontmost_bundle_id()
	if cstr == nil {
		return "", nil
	}
	defer C.free(unsafe.Pointer(cstr))
	return C.GoString(cstr), nil
}

// currentBundleID returns ontext's own bundle id, or "" if unbundled (e.g.
// running via `wails dev`).
func currentBundleID() string {
	cstr := C.focus_current_bundle_id()
	if cstr == nil {
		return ""
	}
	defer C.free(unsafe.Pointer(cstr))
	return C.GoString(cstr)
}

func activateBundleID(bundleID string) error {
	cstr := C.CString(bundleID)
	defer C.free(unsafe.Pointer(cstr))
	if C.focus_activate_bundle_id(cstr) == 0 {
		return errors.New("focus: no running app found for bundle id " + bundleID)
	}
	return nil
}

func isAccessibilityTrusted() bool {
	return C.focus_is_accessibility_trusted() != 0
}

func requestAccessibilityPermission() {
	C.focus_request_accessibility_permission()
}
