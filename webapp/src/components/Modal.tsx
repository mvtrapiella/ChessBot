import type { ReactNode } from 'react'
import styles from './Modal.module.css'

type ModalProps = {
    open: boolean
    children: ReactNode
}

function Modal({ open, children }: Readonly<ModalProps>) {
    if (!open) {
        return null
    }

    return (
        <div className={styles.backdrop}>
            <div className={styles.box}>{children}</div>
        </div>
    )
}

export default Modal
