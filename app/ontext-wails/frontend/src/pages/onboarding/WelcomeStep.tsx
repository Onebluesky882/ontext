interface Props {
  onNext: () => void
}

function IconMic() {
  return (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
      <rect x="9" y="2" width="6" height="12" rx="3" />
      <path d="M5 10a7 7 0 0 0 14 0" />
      <line x1="12" y1="17" x2="12" y2="21" />
      <line x1="8" y1="21" x2="16" y2="21" />
    </svg>
  )
}

function IconKeyboard() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
      <rect x="2" y="6" width="20" height="12" rx="2" />
      <path d="M6 10h.01M10 10h.01M14 10h.01M18 10h.01M8 14h8" />
    </svg>
  )
}

function IconZap() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" aria-hidden>
      <path d="M13 2L4.09 12.11A1 1 0 0 0 5 14h5.5l-1.5 8 9.41-10.11A1 1 0 0 0 17.5 10H12l1-8z" />
    </svg>
  )
}

function IconGlobe() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
      <circle cx="12" cy="12" r="10" />
      <path d="M12 2a14.5 14.5 0 0 1 0 20A14.5 14.5 0 0 1 12 2" />
      <line x1="2" y1="12" x2="22" y2="12" />
    </svg>
  )
}

export function WelcomeStep({ onNext }: Props) {
  return (
    <div className="ob-step ob-welcome">
      <div className="ob-welcome__icon-wrap">
        <IconMic />
      </div>

      <h1 className="ob-welcome__title">Meet ontext</h1>
      <p className="ob-welcome__subtitle">
        Your voice, typed instantly — into any app.
      </p>

      <ul className="ob-feature-list">
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon"><IconKeyboard /></span>
          <div>
            <strong>Hands-free typing</strong>
            <p>Hold your hotkey, speak, release — text appears wherever your cursor is.</p>
          </div>
        </li>
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon"><IconZap /></span>
          <div>
            <strong>Real-time transcription</strong>
            <p>Powered by Whisper — fast, accurate, and private.</p>
          </div>
        </li>
        <li className="ob-feature-list__item">
          <span className="ob-feature-list__icon"><IconGlobe /></span>
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
