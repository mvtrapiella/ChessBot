import styles from './Setup.module.css'

const MIN_DEPTH = 1
const MAX_DEPTH = 6

type DepthSelectorProps = {
    depth: number
    onChange: (depth: number) => void
    disabled?: boolean
}

function clamp(value: number): number {
    return Math.min(MAX_DEPTH, Math.max(MIN_DEPTH, value))
}

function DepthSelector({ depth, onChange, disabled = false }: Readonly<DepthSelectorProps>) {
    return (
        <div className={[styles.section, disabled ? styles.sectionDisabled : ''].filter(Boolean).join(' ')}>
            <span className={styles.label}>Depth</span>
            <div className={styles.depthSpinner}>
                <button
                    type="button"
                    className={styles.depthSpinnerButton}
                    onClick={() => onChange(clamp(depth - 1))}
                    disabled={disabled || depth <= MIN_DEPTH}
                    aria-label="Decrease depth"
                >
                    −
                </button>
                <input
                    className={styles.depthSpinnerInput}
                    type="number"
                    min={MIN_DEPTH}
                    max={MAX_DEPTH}
                    value={depth}
                    onChange={(event) => onChange(clamp(Number(event.target.value) || MIN_DEPTH))}
                    disabled={disabled}
                />
                <button
                    type="button"
                    className={styles.depthSpinnerButton}
                    onClick={() => onChange(clamp(depth + 1))}
                    disabled={disabled || depth >= MAX_DEPTH}
                    aria-label="Increase depth"
                >
                    +
                </button>
            </div>
        </div>
    )
}

export default DepthSelector
