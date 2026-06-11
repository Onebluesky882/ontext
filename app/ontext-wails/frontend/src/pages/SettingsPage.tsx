import { useState } from 'react'
import { useSettings } from '../hooks/useSettings'

export function SettingsPage() {
  const { settings, update } = useSettings()
  const [hotkeyLabel, setHotkeyLabel] = useState(settings.hotkeyLabel)

  const save = () => update({ hotkeyLabel })

  return (
    <main className="settings-page">
      <h2 className="settings-page__title">Settings</h2>

      <section className="settings-section">
        <h3 className="settings-section__heading">Hotkey Display</h3>
        <p className="settings-section__description">
          Label shown in the UI. The actual hotkey is configured in the backend (default: Option+Space).
        </p>
        <div className="settings-section__row">
          <label htmlFor="hotkey-label">Label</label>
          <input
            id="hotkey-label"
            className="input"
            value={hotkeyLabel}
            onChange={(e) => setHotkeyLabel(e.currentTarget.value)}
            placeholder="e.g. ⌥Space"
          />
          <button className="btn btn--primary" onClick={save}>
            Save
          </button>
        </div>
      </section>

      <section className="settings-section">
        <h3 className="settings-section__heading">API Key</h3>
        <p className="settings-section__description">
          The Groq API key is loaded from the <code>VITE_GROQ</code> environment variable.
          Set it in <code>app/ontext/.env</code>:
        </p>
        <pre className="settings-section__code">VITE_GROQ=gsk_…</pre>
      </section>
    </main>
  )
}
