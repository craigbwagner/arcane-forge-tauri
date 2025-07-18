import './App.css'
import { Route, Routes } from 'react-router-dom'
import Dashboard from './pages/Dashboard'
import PageLayout from './layouts/PageLayout'
import CharactersList from './pages/CharactersList'

function App() {
  return (
    <Routes>
      <Route path="/" element={<PageLayout />}>
        <Route index element={<Dashboard />} />
        <Route path="characters" element={<CharactersList />} />
      </Route>
    </Routes>
  )
}

export default App
