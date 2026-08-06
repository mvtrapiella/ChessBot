import styles from './AppHeader.module.css'

function AppHeader() {
    return (
        <header className={styles.header}>
            <h1 className={styles.wordmark}>ATLAS</h1>
            <p className={styles.tagline}>Chess engine — negamax, depth-limited</p>
            <div className={styles.rule} />
        </header>
    )
}

export default AppHeader
