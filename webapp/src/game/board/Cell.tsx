import type { MouseEvent } from 'react'
import bB from '../../assets/bB.svg'
import bK from '../../assets/bK.svg'
import bN from '../../assets/bN.svg'
import bP from '../../assets/bP.svg'
import bQ from '../../assets/bQ.svg'
import bR from '../../assets/bR.svg'
import wB from '../../assets/wB.svg'
import wK from '../../assets/wK.svg'
import wN from '../../assets/wN.svg'
import wP from '../../assets/wP.svg'
import wQ from '../../assets/wQ.svg'
import wR from '../../assets/wR.svg'
import styles from './Board.module.css'

// Matches board_backend's piece codes (types.rs): 0 = empty, 1-6 = white, 7-12 = black
const PIECE_ASSETS: Record<number, string> = {
    1: wP, 2: wR, 3: wN, 4: wB, 5: wQ, 6: wK,
    7: bP, 8: bR, 9: bN, 10: bB, 11: bQ, 12: bK,
}

type CellProps = {
    piece: number
    isLight: boolean
    isSelected: boolean
    isHighlighted: boolean
    isMarked: boolean
    onClick: () => void
    onMouseDown: (event: MouseEvent) => void
    onMouseUp: (event: MouseEvent) => void
}

function Cell({ piece, isLight, isSelected, isHighlighted, isMarked, onClick, onMouseDown, onMouseUp }: Readonly<CellProps>) {
    const asset = PIECE_ASSETS[piece]

    const className = [
        styles.cell,
        isLight ? styles.cellLight : styles.cellDark,
        isSelected ? styles.cellSelected : '',
        isMarked ? styles.cellMarked : '',
    ].filter(Boolean).join(' ')

    return (
        <div
            className={className}
            onClick={onClick}
            onMouseDown={onMouseDown}
            onMouseUp={onMouseUp}
            onDragStart={(event) => event.preventDefault()}
        >
            {isHighlighted && <div className={styles.cellHighlight} />}
            {asset && <img src={asset} alt="" className={styles.cellPiece} draggable={false} />}
        </div>
    )
}

export default Cell
