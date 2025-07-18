import { useState } from 'react'
import './App.css'
import { Route, Routes } from 'react-router-dom'
import Dashboard from './pages/Dashboard'
import PageLayout from './layouts/PageLayout'

function App() {
  return (
    <Routes>
      <Route path="/" element={<PageLayout />}>
        <Route index element={<Dashboard />} />
      </Route>
    </Routes>
  )
}

export default App
