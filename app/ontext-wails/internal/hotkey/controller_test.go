package hotkey

import (
	"errors"
	"sync"
	"testing"
	"time"
)

type fakeListener struct {
	keydown     chan struct{}
	keyup       chan struct{}
	registerErr error

	mu           sync.Mutex
	registered   bool
	unregistered bool
}

func newFakeListener() *fakeListener {
	return &fakeListener{
		keydown: make(chan struct{}, 1),
		keyup:   make(chan struct{}, 1),
	}
}

func (f *fakeListener) Register() error {
	if f.registerErr != nil {
		return f.registerErr
	}
	f.mu.Lock()
	f.registered = true
	f.mu.Unlock()
	return nil
}

func (f *fakeListener) KeyDown() <-chan struct{} { return f.keydown }
func (f *fakeListener) KeyUp() <-chan struct{}   { return f.keyup }

func (f *fakeListener) Unregister() {
	f.mu.Lock()
	f.unregistered = true
	f.mu.Unlock()
}

type fakeToggler struct {
	mu      sync.Mutex
	starts  int
	stops   int
	started chan struct{}
	stopped chan struct{}
}

func newFakeToggler() *fakeToggler {
	return &fakeToggler{started: make(chan struct{}, 8), stopped: make(chan struct{}, 8)}
}

func (f *fakeToggler) Start() {
	f.mu.Lock()
	f.starts++
	f.mu.Unlock()
	f.started <- struct{}{}
}

func (f *fakeToggler) Stop() {
	f.mu.Lock()
	f.stops++
	f.mu.Unlock()
	f.stopped <- struct{}{}
}

func waitFor(t *testing.T, ch <-chan struct{}) {
	t.Helper()
	select {
	case <-ch:
	case <-time.After(time.Second):
		t.Fatal("timed out waiting for event")
	}
}

func TestControllerHoldToTalkStartsAndStops(t *testing.T) {
	listener := newFakeListener()
	toggler := newFakeToggler()
	c := NewController(listener, toggler)

	sessions := make(chan Session, 1)
	c.OnSession = func(s Session) { sessions <- s }

	if err := c.Start(); err != nil {
		t.Fatalf("Start: %v", err)
	}
	defer c.Close()

	listener.keydown <- struct{}{}
	waitFor(t, toggler.started)

	time.Sleep(10 * time.Millisecond)

	listener.keyup <- struct{}{}
	waitFor(t, toggler.stopped)

	select {
	case session := <-sessions:
		if !session.EndedAt.After(session.StartedAt) {
			t.Fatalf("expected EndedAt after StartedAt, got %v -> %v", session.StartedAt, session.EndedAt)
		}
		if session.DurationMs() <= 0 {
			t.Fatalf("expected positive DurationMs, got %d", session.DurationMs())
		}
	case <-time.After(time.Second):
		t.Fatal("timed out waiting for session report")
	}

	toggler.mu.Lock()
	defer toggler.mu.Unlock()
	if toggler.starts != 1 || toggler.stops != 1 {
		t.Fatalf("got starts=%d stops=%d, want 1 and 1", toggler.starts, toggler.stops)
	}
}

func TestControllerIgnoresRepeatedKeyDown(t *testing.T) {
	listener := newFakeListener()
	toggler := newFakeToggler()
	c := NewController(listener, toggler)

	if err := c.Start(); err != nil {
		t.Fatalf("Start: %v", err)
	}
	defer c.Close()

	listener.keydown <- struct{}{}
	waitFor(t, toggler.started)

	// A second key-down while still holding (e.g. OS auto-repeat) must not
	// start a second session.
	listener.keydown <- struct{}{}
	time.Sleep(20 * time.Millisecond)

	listener.keyup <- struct{}{}
	waitFor(t, toggler.stopped)

	toggler.mu.Lock()
	defer toggler.mu.Unlock()
	if toggler.starts != 1 || toggler.stops != 1 {
		t.Fatalf("got starts=%d stops=%d, want 1 and 1", toggler.starts, toggler.stops)
	}
}

func TestControllerRegisterErrorFallsBack(t *testing.T) {
	listener := newFakeListener()
	listener.registerErr = errors.New("hotkey already in use")
	toggler := newFakeToggler()
	c := NewController(listener, toggler)

	err := c.Start()
	if err == nil {
		t.Fatal("expected error from Start when registration fails")
	}

	// No event handling should occur; toggler must remain untouched.
	toggler.mu.Lock()
	defer toggler.mu.Unlock()
	if toggler.starts != 0 || toggler.stops != 0 {
		t.Fatalf("got starts=%d stops=%d, want 0 and 0", toggler.starts, toggler.stops)
	}
}

func TestControllerCloseUnregisters(t *testing.T) {
	listener := newFakeListener()
	toggler := newFakeToggler()
	c := NewController(listener, toggler)

	if err := c.Start(); err != nil {
		t.Fatalf("Start: %v", err)
	}
	c.Close()

	listener.mu.Lock()
	defer listener.mu.Unlock()
	if !listener.unregistered {
		t.Fatal("expected listener to be unregistered after Close")
	}
}
