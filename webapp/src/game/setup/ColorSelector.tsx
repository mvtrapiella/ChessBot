import wK from '../../assets/wK.svg'
import bK from '../../assets/bK.svg'
import './Setup.css'

export type PlayerColorChoice = 'white' | 'random' | 'black'

type ColorSelectorProps = {
    selected: PlayerColorChoice
    onChange: (choice: PlayerColorChoice) => void
}

function tileClassName(isActive: boolean, extra = ''): string {
    return ['color-tile', extra, isActive ? 'color-tile-active' : ''].filter(Boolean).join(' ')
}

function ColorSelector({ selected, onChange }: Readonly<ColorSelectorProps>) {
    return (
        <div className="setup-section">
            <span className="setup-label">Color</span>
            <div className="color-tiles">
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
                    className={tileClassName(selected === 'random', 'color-tile-random')}
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
