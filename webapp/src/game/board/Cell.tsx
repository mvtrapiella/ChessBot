import { useLayoutEffect, useRef, type MouseEvent } from 'react'
import { PIECE_ASSETS } from './pieceAssets'
import styles from './Board.module.css'

type CellProps = {
    piece: number
    isLight: boolean
    isSelected: boolean
    isHighlighted: boolean
    isMarked: boolean
    isChecked: boolean
    isIllegalFlash: boolean
    squareIndex: number
    // Set only on the destination square of a just-made move, in grid cells
    // (not pixels), so the piece can FLIP-animate in from where it came.
    animateFrom: { dx: number; dy: number } | null
    onClick: () => void
    onMouseDown: (event: MouseEvent) => void
    onMouseUp: (event: MouseEvent) => void
}

function Cell({
    piece,
    isLight,
    isSelected,
    isHighlighted,
    isMarked,
    isChecked,
    isIllegalFlash,
    squareIndex,
    animateFrom,
    onClick,
    onMouseDown,
    onMouseUp,
}: Readonly<CellProps>) {
    const asset = PIECE_ASSETS[piece]
    const pieceRef = useRef<HTMLImageElement>(null)

    useLayoutEffect(() => {
        const el = pieceRef.current
        if (!animateFrom || !el) return

        el.style.transition = 'none'
        el.style.transform = `translate(${animateFrom.dx * 100}%, ${animateFrom.dy * 100}%)`
        // Force a reflow so the browser registers that starting position
        // before we animate away from it -- otherwise both style writes
        // land in the same frame and there's nothing to transition from.
        void el.offsetWidth

        const frame = requestAnimationFrame(() => {
            el.style.transition = 'transform 180ms ease-out'
            el.style.transform = 'translate(0, 0)'
        })

        return () => cancelAnimationFrame(frame)
    }, [animateFrom?.dx, animateFrom?.dy, piece])

    const className = [
        styles.cell,
        isLight ? styles.cellLight : styles.cellDark,
        isSelected ? styles.cellSelected : '',
        isMarked ? styles.cellMarked : '',
        isChecked ? styles.cellChecked : '',
        isIllegalFlash ? styles.cellFlash : '',
    ].filter(Boolean).join(' ')

    return (
        <div
            className={className}
            data-square={squareIndex}
            onClick={onClick}
            onMouseDown={onMouseDown}
            onMouseUp={onMouseUp}
            onDragStart={(event) => event.preventDefault()}
        >
            {isHighlighted && <div className={styles.cellHighlight} />}
            {asset && (
                <img ref={pieceRef} src={asset} alt="" className={styles.cellPiece} draggable={false} />
            )}
        </div>
    )
}

export default Cell
