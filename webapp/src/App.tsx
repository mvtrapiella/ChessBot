import { BrowserRouter, Route, Routes } from 'react-router-dom'
import './App.css'
import MainWindow from './game/MainWindow'
import GameWindow from './game/GameWindow'

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path = "/" element = {<MainWindow/>}/>
        <Route path = "/game/:gameId" element = {<GameWindow/>}/>
      </Routes>
    </BrowserRouter>
      
  )
}

export default App
