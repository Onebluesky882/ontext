import { useEffect } from 'react'
import { StartPipeline, StopRecording } from '../../wailsjs/go/main/App'
import { EventsOn, EventsOff } from '../../wailsjs/runtime/runtime'
import { useAppStore, type PipelineStatus } from '../store/appStore'

export function usePipeline() {
  const { status, setRunning, setStatus, setDone, reset } = useAppStore()

  useEffect(() => {
    EventsOn('status', (status: PipelineStatus) => setStatus(status))
    return () => EventsOff('status')
  }, [setStatus])

  const start = async () => {
    if (status === 'running') return
    setRunning()
    try {
      const result = await StartPipeline()
      setDone(result)
    } catch (err) {
      setDone({ success: false, error: String(err) })
    }
  }

  const stop = async () => {
    if (status !== 'running') return
    try {
      await StopRecording()
    } catch {
      // ignore — the pipeline will surface any error through start()'s setDone
    }
  }

  return { status, start, stop, reset }
}
