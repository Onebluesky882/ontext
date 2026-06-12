import { usePipeline } from '../../hooks/usePipeline'
import { useAppStore } from '../../store/appStore'
import { StatusBadge } from '../../components/StatusBadge'

interface Props {
  onNext: () => void
  onBack: () => void
}

export function HotkeyStatusPage({ onNext, onBack }: Props) {
  const { status, start, stop } = usePipeline()
  const { hotkeyUnavailable } = useAppStore()

  return (
    <div className="ob-step">
      <h2 className="ob-step__heading">Hotkey Status</h2>
      <p className="ob-step__description">
        Hold the global hotkey (Cmd+Shift+Space on macOS, Ctrl+Shift+Space on Windows) to
        start recording. Release it to stop.
      </p>

      <section className="main-page__status">
        <StatusBadge status={status} />
        {status === 'running' && <p className="main-page__hint">Recording… release the hotkey to stop</p>}
        {status === 'idle' && <p className="main-page__hint">Idle — hold the hotkey or click Start</p>}
      </section>

      {hotkeyUnavailable && (
        <p className="ob-permission__hint">
          Global hotkey unavailable ({hotkeyUnavailable}). You can still use the buttons below.
        </p>
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
        <button className="btn btn--primary" onClick={onNext}>Continue</button>
      </div>
    </div>
  )
}
