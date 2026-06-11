//go:build darwin

package clipboard

/*
#include <pthread.h>
#include <dispatch/dispatch.h>

extern void goRunOnMain(void *context);

static void runOnMainThreadSync(void *context) {
	if (pthread_main_np()) {
		goRunOnMain(context);
	} else {
		dispatch_sync_f(dispatch_get_main_queue(), context, goRunOnMain);
	}
}
*/
import "C"

import (
	"runtime/cgo"
	"unsafe"
)

//export goRunOnMain
func goRunOnMain(context unsafe.Pointer) {
	h := *(*cgo.Handle)(context)
	h.Value().(func())()
}

// runOnMainThread synchronously runs fn on the OS main thread. Carbon/TSM
// keyboard-layout APIs (used by robotgo's KeyTap) assert they run on the
// main dispatch queue and crash with SIGTRAP otherwise.
func runOnMainThread(fn func()) {
	h := cgo.NewHandle(fn)
	defer h.Delete()
	C.runOnMainThreadSync(unsafe.Pointer(&h))
}
