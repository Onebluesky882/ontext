import { invoke } from '@tauri-apps/api/core'
import type { PasteResult } from '../types/transcript'
import { useAppStore } from '../store/appStore'

export function usePipeline() {
  const { status, setRunning, setDone, reset } = useAppStore()

  const start = async () => {
    if (status === 'running') return
    setRunning()
    try {
      const result = await invoke<PasteResult>('run_pipeline')
      // PasteResult doesn't carry the text back — show generic success
      setDone(result)
    } catch (err) {
      setDone({ success: false, error: String(err) })
    }
  }

  return { status, start, reset }
}
