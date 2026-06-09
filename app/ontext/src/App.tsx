import { useState } from 'react'
import { NavBar } from './components/NavBar'
import { MainPage } from './pages/MainPage'
import { SettingsPage } from './pages/SettingsPage'
import './App.css'

type Page = 'main' | 'settings'

function App() {
  const [page, setPage] = useState<Page>('main')

  return (
    <div className="app">
      <NavBar page={page} onNavigate={setPage} />
      {page === 'main' ? <MainPage /> : <SettingsPage />}
    </div>
  )
}

export default App
