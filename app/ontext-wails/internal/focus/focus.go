// Package focus tracks the frontmost application and reactivates it before
// a paste, so transcribed text lands in the app the user was using rather
// than ontext's own window. Platform-specific bindings (macOS: cgo +
// AppKit/CoreFoundation) live in focus_darwin.go; other platforms get a
// no-op implementation in focus_other.go.
package focus

import (
	"context"
	"sync"
	"time"
)

// SettleDelay is how long to wait after reactivating an app before sending
// the paste keystroke, giving the OS time to finish the focus transition.
const SettleDelay = 100 * time.Millisecond

const pollInterval = 300 * time.Millisecond

// Manager tracks the frontmost application (excluding ontext itself) and can
// reactivate it on demand.
type Manager struct {
	mu      sync.Mutex
	lastApp string
	self    string
	cancel  context.CancelFunc
}

// New returns a Manager that ignores ontext's own bundle id when tracking
// the frontmost application.
func New() *Manager {
	return &Manager{self: currentBundleID()}
}

// Start begins polling the frontmost application in the background. It is a
// no-op if already started.
func (m *Manager) Start() {
	m.mu.Lock()
	if m.cancel != nil {
		m.mu.Unlock()
		return
	}
	ctx, cancel := context.WithCancel(context.Background())
	m.cancel = cancel
	m.mu.Unlock()

	go m.poll(ctx)
}

// Stop ends the background polling, if running.
func (m *Manager) Stop() {
	m.mu.Lock()
	defer m.mu.Unlock()
	if m.cancel != nil {
		m.cancel()
		m.cancel = nil
	}
}

func (m *Manager) poll(ctx context.Context) {
	ticker := time.NewTicker(pollInterval)
	defer ticker.Stop()
	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			bid, err := frontmostBundleID()
			if err != nil || bid == "" || bid == m.self {
				continue
			}
			m.mu.Lock()
			m.lastApp = bid
			m.mu.Unlock()
		}
	}
}

// LastFocusedApp returns the bundle id of the most recently observed
// frontmost application that wasn't ontext itself, or "" if none has been
// observed yet.
func (m *Manager) LastFocusedApp() string {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.lastApp
}

// Activate brings the app with the given bundle id to the foreground and
// waits SettleDelay for the OS to finish the focus transition. It is a no-op
// if bundleID is empty.
func (m *Manager) Activate(bundleID string) error {
	if bundleID == "" {
		return nil
	}
	if err := activateBundleID(bundleID); err != nil {
		return err
	}
	time.Sleep(SettleDelay)
	return nil
}

// IsAccessibilityTrusted reports whether ontext currently has macOS
// Accessibility permission. On non-macOS platforms it always returns true.
func IsAccessibilityTrusted() bool {
	return isAccessibilityTrusted()
}

// RequestAccessibilityPermission prompts the user to grant Accessibility
// permission to ontext (macOS only; no-op elsewhere).
func RequestAccessibilityPermission() {
	requestAccessibilityPermission()
}
