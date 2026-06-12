import { usePipeline } from '../../hooks/usePipeline'
import { useAppStore } from '../../store/appStore'
import { StatusBadge } from '../../components/StatusBadge'

interface Props {
  onBack: () => void
}

export function LiveTranscriptPage({ onBack }: Props) {
  const { status, start, stop } = usePipeline()
  const { partialTranscript, lastText, errorMessage } = useAppStore()

  const text = status === 'running' ? partialTranscript : (lastText ?? partialTranscript)

  return (
    <div className="ob-step">
      <h2 className="ob-step__heading">Live Transcript</h2>
      <p className="ob-step__description">
        While the hotkey is held, the transcript streams here as it's recognized.
      </p>

      <section className="main-page__status">
        <StatusBadge status={status} />
      </section>

      <div className="flow-transcript__box" aria-live="polite">
        {text || <span className="flow-transcript__placeholder">Transcript will appear here…</span>}
      </div>

      {status === 'error' && errorMessage && (
        <p className="ob-permission__hint">Error: {errorMessage}</p>
      )}

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

      <div className="ob-step__footer">
        <button className="btn btn--secondary" onClick={onBack}>Back</button>
      </div>
    </div>
  )
}
