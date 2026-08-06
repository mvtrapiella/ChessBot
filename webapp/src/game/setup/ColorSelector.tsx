import wK from '../../assets/wK.svg'
import bK from '../../assets/bK.svg'
import styles from './Setup.module.css'

export type PlayerColorChoice = 'white' | 'random' | 'black'

type ColorSelectorProps = {
    selected: PlayerColorChoice
    onChange: (choice: PlayerColorChoice) => void
}

function tileClassName(isActive: boolean, extra?: string): string {
    return [styles.colorTile, extra, isActive ? styles.colorTileActive : '']
        .filter(Boolean)
        .join(' ')
}

function ColorSelector({ selected, onChange }: Readonly<ColorSelectorProps>) {
    return (
        <div className={styles.section}>
            <span className={styles.label}>Color</span>
            <div className={styles.colorTiles}>
                <button
                    type="button"
                    className={tileClassName(selected === 'white')}
                    onClick={() => onChange('white')}
                    aria-label="Play as White"
                >
                    <img src={wK} alt="" />
                </button>
                <button
                    type="button"
                    className={tileClassName(selected === 'random', styles.colorTileRandom)}
                    onClick={() => onChange('random')}
                    aria-label="Play as a random color"
                />
                <button
                    type="button"
                    className={tileClassName(selected === 'black')}
                    onClick={() => onChange('black')}
                    aria-label="Play as Black"
                >
                    <img src={bK} alt="" />
                </button>
            </div>
        </div>
    )
}

export default ColorSelector
