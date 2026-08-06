import type { MoveRecordDTO } from '../api/gameApi'
import styles from './MoveHistory.module.css'

type MoveHistoryProps = {
    moveHistory: MoveRecordDTO[]
    viewIndex: number
    onSelectIndex: (index: number) => void
}

function plyButtonClassName(isActive: boolean): string {
    return [styles.ply, isActive ? styles.plyActive : ''].filter(Boolean).join(' ')
}

function MoveHistory({ moveHistory, viewIndex, onSelectIndex }: Readonly<MoveHistoryProps>) {
    if (moveHistory.length === 0) {
        return <p className={styles.empty}>No moves yet</p>
    }

    const rows = Array.from({ length: Math.ceil(moveHistory.length / 2) }, (_, row) => row)

    return (
        <div className={styles.list}>
            {rows.map((row) => {
                const whiteIndex = row * 2
                const blackIndex = row * 2 + 1
                const white = moveHistory[whiteIndex]
                const black = moveHistory[blackIndex] as MoveRecordDTO | undefined

                return (
                    <div className={styles.row} key={row}>
                        <span className={styles.moveNumber}>{row + 1}.</span>
                        <button
                            type="button"
                            className={plyButtonClassName(viewIndex === whiteIndex)}
                            onClick={() => onSelectIndex(whiteIndex)}
                        >
                            {white.notation}
                        </button>
                        {black && (
                            <button
                                type="button"
                                className={plyButtonClassName(viewIndex === blackIndex)}
                                onClick={() => onSelectIndex(blackIndex)}
                            >
                                {black.notation}
                            </button>
                        )}
                    </div>
                )
            })}
        </div>
    )
}

export default MoveHistory
