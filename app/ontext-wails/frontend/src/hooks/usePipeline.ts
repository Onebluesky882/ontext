import { invoke } from '@tauri-apps/api/core'
import type { PasteResult } from '../types/transcript'
import { useAppStore } from '../store/appStore'

export function usePipeline() {
  const { status, setRunning, setDone, reset } = useAppStore()

  const start = async () => {
    if (status === 'running') return
    setRunning()
    try {
      const result = await invoke<PasteResult>('start_pipeline')
      setDone(result)
    } catch (err) {
      setDone({ success: false, error: String(err) })
    }
  }

  const stop = async () => {
    if (status !== 'running') return
    try {
      await invoke('stop_recording')
    } catch {
      // ignore — the pipeline will surface any error through start()'s setDone
    }
  }

  return { status, start, stop, reset }
}
