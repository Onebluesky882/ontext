import { useEffect, useState } from 'react'
import {
  GetPermissionStatus,
  RequestAccessibilityPermission,
  RequestMicrophonePermission,
} from '../../../wailsjs/go/main/App'
import { BrowserOpenURL } from '../../../wailsjs/runtime/runtime'
import type { MicrophonePermission } from '../../types/events'

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

function IconMic() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
      <rect x="9" y="2" width="6" height="11" rx="3" />
      <path d="M5 10a7 7 0 0 0 14 0" />
      <path d="M12 17v4" />
      <path d="M8 21h8" />
    </svg>
  )
}

export function PermissionStep({ onDone, onBack }: Props) {
  const [opened, setOpened] = useState(false)
  const [accessibilityGranted, setAccessibilityGranted] = useState(false)
  const [micPermission, setMicPermission] = useState<MicrophonePermission>('not_determined')

  const refreshStatus = async () => {
    try {
      const status = await GetPermissionStatus()
      setAccessibilityGranted(status.accessibility)
      setMicPermission(status.microphone as MicrophonePermission)
    } catch {
      // GetPermissionStatus is unavailable (e.g. non-macOS); treat as granted
      // so the flow isn't blocked.
      setAccessibilityGranted(true)
      setMicPermission('authorized')
    }
  }

  useEffect(() => {
    refreshStatus()
    requestMic()
  }, [])

  const requestMic = async () => {
    try {
      const result = await RequestMicrophonePermission()
      setMicPermission(result as MicrophonePermission)
    } catch {
      // ignore - non-macOS or binding unavailable
    }
  }

  const openSettings = async () => {
    try {
      await RequestAccessibilityPermission()
    } catch {
      BrowserOpenURL('x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility')
    }
    setOpened(true)
    await refreshStatus()
  }

  const openMicSettings = () => {
    BrowserOpenURL('x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone')
  }

  const canContinue = opened || accessibilityGranted

  return (
    <div className="ob-step ob-permission">
      <div className="ob-permission__icon-wrap">
        <IconLock />
      </div>

      <h2 className="ob-step__heading">Permissions</h2>
      <p className="ob-step__description">
        ontext needs Microphone access to hear you and Accessibility access to
        type transcribed text into any app on your Mac. Your audio is never
        uploaded — everything runs locally.
      </p>

      <div className="ob-permission__section">
        <div className="ob-permission__section-header">
          <IconMic />
          <h3>Microphone</h3>
        </div>
        {micPermission === 'authorized' && (
          <p className="ob-permission__status ob-permission__status--ok">✓ Microphone access granted.</p>
        )}
        {micPermission === 'not_determined' && (
          <p className="ob-permission__status">Requesting microphone access...</p>
        )}
        {(micPermission === 'denied' || micPermission === 'restricted') && (
          <>
            <p className="ob-permission__status ob-permission__status--warn">
              Microphone access was denied. ontext won't be able to hear you until this is enabled.
            </p>
            <div className="ob-permission__actions">
              <button className="btn btn--secondary" onClick={openMicSettings}>
                Open Microphone Settings
              </button>
            </div>
          </>
        )}
      </div>

      <div className="ob-permission__section">
        <div className="ob-permission__section-header">
          <IconLock />
          <h3>Accessibility</h3>
        </div>

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

        {accessibilityGranted ? (
          <p className="ob-permission__status ob-permission__status--ok">✓ Accessibility access granted.</p>
        ) : (
          <div className="ob-permission__actions">
            <button className="btn btn--secondary" onClick={openSettings}>
              Open System Settings
            </button>
          </div>
        )}

        {opened && !accessibilityGranted && (
          <p className="ob-permission__hint">
            ✓ Once you've enabled Accessibility for ontext, click continue below.
          </p>
        )}
      </div>

      <div className="ob-step__footer">
        <button className="btn btn--secondary" onClick={onBack}>Back</button>
        <button
          className="btn btn--primary"
          onClick={onDone}
          disabled={!canContinue}
          title={!canContinue ? 'Click "Open System Settings" first' : undefined}
        >
          I've Granted Access
        </button>
      </div>
    </div>
  )
}
