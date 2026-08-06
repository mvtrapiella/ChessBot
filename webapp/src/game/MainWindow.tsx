import { useState } from 'react'
import Board from './board/Board'
import { INITIAL_SQUARES } from './board/initialPosition'

// Temporary: just enough state to render the board and see clicks register.
// Real click-to-move logic (and the depth/color selector) still to come.
function MainWindow() {
    const [squares] = useState(INITIAL_SQUARES)
    const [selectedSquare, setSelectedSquare] = useState<number | null>(null)

    const handleSquareClick = (index: number) => {
        setSelectedSquare((current) => (current === index ? null : index))
    }

    return (
        <Board
            squares={squares}
            selectedSquare={selectedSquare}
            onSquareClick={handleSquareClick}
        />
    )
}

export default MainWindow
