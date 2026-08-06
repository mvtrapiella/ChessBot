import './Setup.css'

const MIN_DEPTH = 1
const MAX_DEPTH = 6

type DepthSelectorProps = {
    depth: number
    onChange: (depth: number) => void
}

function clamp(value: number): number {
    return Math.min(MAX_DEPTH, Math.max(MIN_DEPTH, value))
}

function DepthSelector({ depth, onChange }: Readonly<DepthSelectorProps>) {
    return (
        <div className="setup-section">
            <span className="setup-label">Depth</span>
            <div className="depth-spinner">
                <button
                    type="button"
                    className="depth-spinner-button"
                    onClick={() => onChange(clamp(depth - 1))}
                    disabled={depth <= MIN_DEPTH}
                >
                    −
                </button>
                <input
                    className="depth-spinner-input"
                    type="number"
                    min={MIN_DEPTH}
                    max={MAX_DEPTH}
                    value={depth}
                    onChange={(event) => onChange(clamp(Number(event.target.value) || MIN_DEPTH))}
                />
                <button
                    type="button"
                    className="depth-spinner-button"
                    onClick={() => onChange(clamp(depth + 1))}
                    disabled={depth >= MAX_DEPTH}
                >
                    +
                </button>
            </div>
        </div>
    )
}

export default DepthSelector
