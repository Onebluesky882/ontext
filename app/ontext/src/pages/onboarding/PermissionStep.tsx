import { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { openUrl } from '@tauri-apps/plugin-opener'

interface Props {
  onDone: () => void
  onBack: () => void
}

function IconLock() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
      <rect x="5" y="11" width="14" height="11" rx="2" />
      <path d="M8 11V7a4 4 0 0 1 8 0v4" />
      <circle cx="12" cy="17" r="1" fill="currentColor" stroke="none" />
    </svg>
  )
}

export function PermissionStep({ onDone, onBack }: Props) {
  const [opened, setOpened] = useState(false)

  const openSettings = async () => {
    try {
      await invoke('request_accessibility_permission')
    } catch {
      try {
        await openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility')
      } catch {
        // show manual path hint below
      }
    }
    setOpened(true)
  }

  return (
    <div className="ob-step ob-permission">
      <div className="ob-permission__icon-wrap">
        <IconLock />
      </div>

      <h2 className="ob-step__heading">Accessibility Access</h2>
      <p className="ob-step__description">
        ontext needs Accessibility permission to type transcribed text into any app on your Mac.
        Your audio is never uploaded — everything runs locally.
      </p>

      <ol className="ob-steps-list">
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">1</span>
          <div>
            Click <strong>"Open System Settings"</strong> — macOS will open Accessibility settings.
          </div>
        </li>
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">2</span>
          <div>
            Toggle <strong>ontext</strong> to <strong>ON</strong>. Enter your password if prompted.
          </div>
        </li>
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">3</span>
          <div>
            Come back here and click <strong>"I've Granted Access"</strong>.
          </div>
        </li>
      </ol>

      <div className="ob-permission__actions">
        <button className="btn btn--primary" onClick={openSettings}>
          Open System Settings
        </button>
      </div>

      {opened && (
        <p className="ob-permission__hint">
          ✓ Once you've enabled Accessibility for ontext, click continue below.
        </p>
      )}

      <div className="ob-step__footer">
        <button className="btn btn--secondary" onClick={onBack}>Back</button>
        <button
          className="btn btn--primary"
          onClick={onDone}
          disabled={!opened}
          title={!opened ? 'Click "Open System Settings" first' : undefined}
        >
          I've Granted Access
        </button>
      </div>
    </div>
  )
}
