interface Language {
  code: string
  label: string
  native: string
  flag: string
}

const LANGUAGES: Language[] = [
  { code: 'en', label: 'English',    native: 'English',    flag: '🇺🇸' },
  { code: 'th', label: 'Thai',       native: 'ภาษาไทย',   flag: '🇹🇭' },
  { code: 'ja', label: 'Japanese',   native: '日本語',      flag: '🇯🇵' },
  { code: 'zh', label: 'Chinese',    native: '中文',        flag: '🇨🇳' },
  { code: 'ko', label: 'Korean',     native: '한국어',      flag: '🇰🇷' },
  { code: 'es', label: 'Spanish',    native: 'Español',    flag: '🇪🇸' },
  { code: 'fr', label: 'French',     native: 'Français',   flag: '🇫🇷' },
  { code: 'de', label: 'German',     native: 'Deutsch',    flag: '🇩🇪' },
  { code: 'pt', label: 'Portuguese', native: 'Português',  flag: '🇧🇷' },
  { code: 'vi', label: 'Vietnamese', native: 'Tiếng Việt', flag: '🇻🇳' },
  { code: 'id', label: 'Indonesian', native: 'Bahasa',     flag: '🇮🇩' },
  { code: 'ru', label: 'Russian',    native: 'Русский',    flag: '🇷🇺' },
]

interface Props {
  selected: string
  onSelect: (code: string) => void
  onNext: () => void
  onBack: () => void
}

export function LanguageStep({ selected, onSelect, onNext, onBack }: Props) {
  return (
    <div className="ob-step ob-language">
      <h2 className="ob-step__heading">Choose Output Language</h2>
      <p className="ob-step__description">
        Select the language you want your speech typed in. You can change this later in Settings.
      </p>

      <div className="ob-lang-grid">
        {LANGUAGES.map((lang) => (
          <button
            key={lang.code}
            className={`ob-lang-card ${selected === lang.code ? 'ob-lang-card--selected' : ''}`}
            onClick={() => onSelect(lang.code)}
          >
            <span className="ob-lang-card__flag">{lang.flag}</span>
            <span className="ob-lang-card__label">{lang.label}</span>
            <span className="ob-lang-card__native">{lang.native}</span>
          </button>
        ))}
      </div>

      <div className="ob-step__footer">
        <button className="btn btn--secondary" onClick={onBack}>Back</button>
        <button className="btn btn--primary" onClick={onNext} disabled={!selected}>
          Continue
        </button>
      </div>
    </div>
  )
}
