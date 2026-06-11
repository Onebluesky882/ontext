package hotkey

import (
	"sync"
	"time"
)

// Toggler is the subset of pipeline.Pipeline the Controller needs. Start
// begins a recording session; Stop ends the session started by Start.
type Toggler interface {
	Start()
	Stop()
}

// Session records the start/end timestamps of one hotkey hold, for
// usage-session reporting (ADR 010: POST /usage/events).
type Session struct {
	StartedAt time.Time
	EndedAt   time.Time
}

// DurationMs returns the session length in milliseconds, as reported to
// POST /usage/events (durationMs = endedAt - startedAt).
func (s Session) DurationMs() int64 {
	return s.EndedAt.Sub(s.StartedAt).Milliseconds()
}

// Controller listens for hold-to-talk hotkey presses: hotkey-down starts a
// recording session via Toggler.Start, hotkey-up stops it via
// Toggler.Stop. Each completed hold is reported via OnSession with its
// start/end timestamps.
type Controller struct {
	listener Listener
	toggler  Toggler

	// OnSession, if set, is called with the timestamps of each completed
	// hotkey hold (key-down to key-up).
	OnSession func(Session)

	mu      sync.Mutex
	holding bool
	started time.Time

	stop chan struct{}
}

// NewController creates a Controller for the given listener and toggler.
func NewController(listener Listener, toggler Toggler) *Controller {
	return &Controller{listener: listener, toggler: toggler}
}

// Start registers the hotkey and begins handling press/release events in
// the background. If registration fails, it returns the error so the
// caller can fall back to button-only operation; the controller does
// nothing further in that case.
func (c *Controller) Start() error {
	if err := c.listener.Register(); err != nil {
		return err
	}

	c.stop = make(chan struct{})
	go c.loop()
	return nil
}

func (c *Controller) loop() {
	for {
		select {
		case <-c.listener.KeyDown():
			c.onKeyDown()
		case <-c.listener.KeyUp():
			c.onKeyUp()
		case <-c.stop:
			return
		}
	}
}

func (c *Controller) onKeyDown() {
	c.mu.Lock()
	defer c.mu.Unlock()

	if c.holding {
		return
	}
	c.holding = true
	c.started = time.Now()
	c.toggler.Start()
}

func (c *Controller) onKeyUp() {
	c.mu.Lock()
	if !c.holding {
		c.mu.Unlock()
		return
	}
	c.holding = false
	session := Session{StartedAt: c.started, EndedAt: time.Now()}
	c.mu.Unlock()

	c.toggler.Stop()

	if c.OnSession != nil {
		c.OnSession(session)
	}
}

// Close unregisters the hotkey and stops handling events.
func (c *Controller) Close() {
	if c.stop != nil {
		close(c.stop)
		c.stop = nil
	}
	c.listener.Unregister()
}
