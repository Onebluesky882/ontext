interface Props {
  page: 'main' | 'settings'
  onNavigate: (page: 'main' | 'settings') => void
}

export function NavBar({ page, onNavigate }: Props) {
  return (
    <nav className="navbar">
      <span className="navbar__title">ontext</span>
      <button
        className={`navbar__icon-btn ${page === 'settings' ? 'navbar__icon-btn--active' : ''}`}
        onClick={() => onNavigate(page === 'settings' ? 'main' : 'settings')}
        aria-label="Settings"
      >
        ⚙
      </button>
    </nav>
  )
}
