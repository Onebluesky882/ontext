import { useEffect, useRef } from 'react'
import { usePipeline } from '../hooks/usePipeline'
import { useAppStore } from '../store/appStore'
import { StatusBadge } from '../components/StatusBadge'
import { ResultCard } from '../components/ResultCard'

export function MainPage() {
  const { status, start, stop, reset } = usePipeline()
  const { lastText, errorMessage } = useAppStore()
  const startedRef = useRef(false)

  // Auto-start recording once on mount
  useEffect(() => {
    if (startedRef.current) return
    startedRef.current = true
    start()
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  // Release mic when window closes / navigates away
  useEffect(() => {
    const handler = () => { stop() }
    window.addEventListener('beforeunload', handler)
    return () => window.removeEventListener('beforeunload', handler)
  }, [stop])

  // Auto-reset after 5 s so the UI stays clean
  useEffect(() => {
    if (status !== 'done' && status !== 'error') return
    const id = setTimeout(reset, 5000)
    return () => clearTimeout(id)
  }, [status, reset])

  return (
    <main className="main-page">
      <section className="main-page__status">
        <StatusBadge status={status} />
        {status === 'running' && (
          <p className="main-page__hint">Recording… click Stop to transcribe</p>
        )}
        {status === 'idle' && (
          <p className="main-page__hint">Click Start Recording to begin</p>
        )}
      </section>

      <section className="main-page__controls">
        {status === 'idle' || status === 'done' || status === 'error' ? (
          <button className="btn btn--primary" onClick={start}>
            Start Recording
          </button>
        ) : (
          <button className="btn btn--secondary" onClick={stop}>
            Stop Recording
          </button>
        )}
      </section>

      <section className="main-page__result">
        <ResultCard status={status} lastText={lastText} errorMessage={errorMessage} />
      </section>
    </main>
  )
}
