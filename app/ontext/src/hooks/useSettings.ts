import { useState } from 'react'

interface Settings {
  hotkeyLabel: string
}

const STORAGE_KEY = 'ontext:settings'
const defaults: Settings = { hotkeyLabel: '⌃Space' }

function load(): Settings {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    return raw ? { ...defaults, ...JSON.parse(raw) } : defaults
  } catch {
    return defaults
  }
}

export function useSettings() {
  const [settings, setSettingsState] = useState<Settings>(load)

  const update = (patch: Partial<Settings>) => {
    const next = { ...settings, ...patch }
    setSettingsState(next)
    localStorage.setItem(STORAGE_KEY, JSON.stringify(next))
  }

  return { settings, update }
}
