interface Props {
  onNext: () => void
}

export function WelcomeStep({ onNext }: Props) {
  return (
    <div className="ob-step ob-welcome">
      <div className="ob-welcome__icon" aria-hidden>🎙️</div>

      <h1 className="ob-welcome__title">Meet ontext</h1>
      <p className="ob-welcome__subtitle">
        Your voice, typed instantly — into any app.
      </p>

      <ul className="ob-feature-list">
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon">⌨️</span>
          <div>
            <strong>Hands-free typing</strong>
            <p>Hold your hotkey, speak, release — text appears wherever your cursor is.</p>
          </div>
        </li>
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon">⚡</span>
          <div>
            <strong>Real-time transcription</strong>
            <p>Powered by Whisper — fast, accurate, and private.</p>
          </div>
        </li>
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon">🌐</span>
          <div>
            <strong>Multi-language support</strong>
            <p>Speak in your language and get output in any language you choose.</p>
          </div>
        </li>
      </ul>

      <button className="btn btn--primary ob-step__cta" onClick={onNext}>
        Get Started
      </button>
    </div>
  )
}
