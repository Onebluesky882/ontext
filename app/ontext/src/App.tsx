import { useState } from 'react'
import { NavBar } from './components/NavBar'
import { MainPage } from './pages/MainPage'
import { SettingsPage } from './pages/SettingsPage'
import { OnboardingPage } from './pages/OnboardingPage'
import { useSettings } from './hooks/useSettings'
import './App.css'

const ONBOARDING_KEY = 'ontext:onboarding-done'

type Page = 'main' | 'settings'

function App() {
  const [onboardingDone, setOnboardingDone] = useState(
    () => localStorage.getItem(ONBOARDING_KEY) === 'true'
  )
  const [page, setPage] = useState<Page>('main')
  const { update } = useSettings()

  const completeOnboarding = (outputLanguage: string) => {
    update({ outputLanguage })
    localStorage.setItem(ONBOARDING_KEY, 'true')
    setOnboardingDone(true)
  }

  if (!onboardingDone) {
    return <OnboardingPage onComplete={completeOnboarding} />
  }

  return (
    <div className="app">
      <NavBar page={page} onNavigate={setPage} />
      {page === 'main' ? <MainPage /> : <SettingsPage />}
    </div>
  )
}

export default App
