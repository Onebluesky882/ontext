import { useState } from 'react'
import { openUrl } from '@tauri-apps/plugin-opener'

interface Props {
  onDone: () => void
  onBack: () => void
}

export function PermissionStep({ onDone, onBack }: Props) {
  const [opened, setOpened] = useState(false)

  const openSettings = async () => {
    try {
      await openUrl('x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility')
    } catch {
      // fallback: show manual path
    }
    setOpened(true)
  }

  return (
    <div className="ob-step ob-permission">
      <div className="ob-permission__icon" aria-hidden>🔐</div>

      <h2 className="ob-step__heading">Accessibility Access Required</h2>
      <p className="ob-step__description">
        ontext needs Accessibility permission to type transcribed text directly into any app on your Mac.
        Your audio is never uploaded — everything runs locally.
      </p>

      <ol className="ob-steps-list">
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">1</span>
          <div>
            Click <strong>"Open System Settings"</strong> below.
          </div>
        </li>
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">2</span>
          <div>
            Go to <strong>Privacy & Security → Accessibility</strong>.
          </div>
        </li>
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">3</span>
          <div>
            Toggle <strong>ontext</strong> to <strong>ON</strong>. Enter your password if prompted.
          </div>
        </li>
        <li className="ob-steps-list__item">
          <span className="ob-steps-list__num">4</span>
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
          Once you've enabled Accessibility for ontext, come back and continue.
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
