import { useEffect, useState } from 'react'
import { RequestAccessibilityPermission } from '../../../wailsjs/go/main/App'
import { BrowserOpenURL } from '../../../wailsjs/runtime/runtime'

interface Props {
  onNext: () => void
}

type PermStatus = 'unknown' | 'granted' | 'denied'

export function PermissionsPage({ onNext }: Props) {
  const [mic, setMic] = useState<PermStatus>('unknown')
  const [accessibility, setAccessibility] = useState<PermStatus>('unknown')

  useEffect(() => {
    navigator.mediaDevices
      ?.getUserMedia({ audio: true })
      .then((stream) => {
        stream.getTracks().forEach((track) => track.stop())
        setMic('granted')
      })
      .catch(() => setMic('denied'))
  }, [])

  const requestMic = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true })
      stream.getTracks().forEach((track) => track.stop())
      setMic('granted')
    } catch {
      setMic('denied')
    }
  }

  const requestAccessibility = async () => {
    try {
      await RequestAccessibilityPermission()
      setAccessibility('granted')
    } catch {
      setAccessibility('denied')
      BrowserOpenURL('x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility')
    }
  }

  return (
    <div className="ob-step">
      <h2 className="ob-step__heading">Permissions</h2>
      <p className="ob-step__description">
        ontext needs microphone access to hear you, and Accessibility access to type
        transcribed text into other apps.
      </p>

      <div className="flow-permission__row">
        <span>Microphone</span>
        <button className="btn btn--secondary" onClick={requestMic}>
          {mic === 'granted' ? 'Granted ✓' : 'Allow Microphone'}
        </button>
      </div>

      <div className="flow-permission__row">
        <span>Accessibility</span>
        <button className="btn btn--secondary" onClick={requestAccessibility}>
          {accessibility === 'granted' ? 'Granted ✓' : 'Open System Settings'}
        </button>
      </div>

      {accessibility === 'denied' && (
        <p className="ob-permission__hint">
          Toggle ontext ON in System Settings → Privacy &amp; Security → Accessibility, then come back.
        </p>
      )}

      <div className="ob-step__footer">
        <button className="btn btn--primary ob-step__cta" onClick={onNext}>
          Continue
        </button>
      </div>
    </div>
  )
}
