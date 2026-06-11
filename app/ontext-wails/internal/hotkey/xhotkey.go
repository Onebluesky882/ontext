package hotkey

import (
	"runtime"

	xhotkey "golang.design/x/hotkey"
)

// XHotkey is a Listener backed by golang.design/x/hotkey. On macOS it
// registers via Carbon's RegisterEventHotKey, which (unlike the rdev-based
// CGEventTap listener used previously) does not require the Accessibility
// permission — see ADR 010 / DECISIONS.md ("Hotkey Reintroduction").
type XHotkey struct {
	hk      *xhotkey.Hotkey
	keydown chan struct{}
	keyup   chan struct{}
	done    chan struct{}
}

// New returns a Listener for the default hold-to-talk hotkey:
// Cmd+Shift+Space on macOS, Ctrl+Shift+Space on Windows/Linux.
func New() *XHotkey {
	mods := []xhotkey.Modifier{xhotkey.ModShift}
	if runtime.GOOS == "darwin" {
		mods = append(mods, xhotkey.ModCmd)
	} else {
		mods = append(mods, xhotkey.ModCtrl)
	}

	return &XHotkey{
		hk:      xhotkey.New(mods, xhotkey.KeySpace),
		keydown: make(chan struct{}, 1),
		keyup:   make(chan struct{}, 1),
	}
}

// Register implements Listener.
func (x *XHotkey) Register() error {
	if err := x.hk.Register(); err != nil {
		return err
	}

	x.done = make(chan struct{})
	go func() {
		for {
			select {
			case <-x.hk.Keydown():
				select {
				case x.keydown <- struct{}{}:
				default:
				}
			case <-x.hk.Keyup():
				select {
				case x.keyup <- struct{}{}:
				default:
				}
			case <-x.done:
				return
			}
		}
	}()

	return nil
}

// KeyDown implements Listener.
func (x *XHotkey) KeyDown() <-chan struct{} {
	return x.keydown
}

// KeyUp implements Listener.
func (x *XHotkey) KeyUp() <-chan struct{} {
	return x.keyup
}

// Unregister implements Listener.
func (x *XHotkey) Unregister() {
	if x.done != nil {
		close(x.done)
		x.done = nil
	}
	_ = x.hk.Unregister()
}
