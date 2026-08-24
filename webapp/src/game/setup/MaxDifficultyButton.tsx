import styles from './Setup.module.css'

type MaxDifficultyButtonProps = {
    active: boolean
    onToggle: () => void
}

function MaxDifficultyButton({ active, onToggle }: Readonly<MaxDifficultyButtonProps>) {
    return (
        <button
            type="button"
            className={[styles.maxDifficultyButton, active ? styles.maxDifficultyButtonActive : '']
                .filter(Boolean)
                .join(' ')}
            onClick={onToggle}
            aria-pressed={active}
        >
            Max Difficulty
        </button>
    )
}

export default MaxDifficultyButton
