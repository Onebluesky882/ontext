import type { PipelineStatus } from '../store/appStore'

interface Props {
  status: PipelineStatus
  lastText: string | null
  errorMessage: string | null
}

export function ResultCard({ status, lastText, errorMessage }: Props) {
  if (status === 'idle' && !lastText) {
    return (
      <div className="result-card result-card--empty">
        <p className="result-card__hint">Start the pipeline to transcribe speech.</p>
      </div>
    )
  }

  if (status === 'error' || errorMessage) {
    return (
      <div className="result-card result-card--error">
        <span className="result-card__label">Error</span>
        <p className="result-card__message">{errorMessage ?? 'Unknown error'}</p>
      </div>
    )
  }

  if (status === 'done') {
    return (
      <div className="result-card result-card--done">
        <span className="result-card__label">Transcribed &amp; pasted</span>
        {lastText && <p className="result-card__message">{lastText}</p>}
      </div>
    )
  }

  return null
}
