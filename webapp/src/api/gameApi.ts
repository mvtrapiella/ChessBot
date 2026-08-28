// Mirrors game_engine/web_server/src/dto.rs -- keep these in sync with the Rust DTOs.
export type ColorDTO = 'WHITE' | 'BLACK'
export type GameStatusDTO = 'IN_PROGRESS' | 'WHITE_WON' | 'BLACK_WON' | 'DRAWN'
export type DrawReasonDTO =
    | 'STALEMATE'
    | 'FIFTY_MOVE_RULE'
    | 'INSUFFICIENT_MATERIAL'
    | 'THREEFOLD_REPETITION'

export type MoveRecordDTO = {
    ply: number
    color: ColorDTO
    notation: string
    squares: number[]
}

export type GameStateDTO = {
    gameId: string
    squares: number[]
    sideToMove: ColorDTO
    userColor: ColorDTO
    status: GameStatusDTO
    drawReason: DrawReasonDTO | null
    moveHistory: MoveRecordDTO[]
}

export type MoveRequest = {
    origin: string
    destination: string
    promotion: string | null
}

export type CreateGameRequest = {
    userColor: ColorDTO
    depth: number
    maxDifficulty: boolean
}

// Relative, not an absolute host:port -- always same-origin with whatever served the
// page itself. In production and the dev-compose setup, nginx reverse-proxies /api/*
// to the backend container (see webapp/nginx.https.conf); for plain `npm run dev`,
// Vite's own dev-server proxy does the equivalent (see vite.config.ts).
const API_BASE_URL = '/api'

async function parseResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
        const body: { error?: string } | null = await response.json().catch(() => null)
        throw new Error(body?.error ?? `Request failed with status ${response.status}`)
    }
    return response.json() as Promise<T>
}

export function createGame(request: CreateGameRequest): Promise<GameStateDTO> {
    return fetch(`${API_BASE_URL}/games`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
    }).then((response) => parseResponse<GameStateDTO>(response))
}

export function getGame(gameId: string): Promise<GameStateDTO> {
    return fetch(`${API_BASE_URL}/games/${gameId}`).then((response) => parseResponse<GameStateDTO>(response))
}

export function getLegalMoves(gameId: string, square: string): Promise<number[]> {
    return fetch(`${API_BASE_URL}/games/${gameId}/legal-moves/${square}`)
        .then((response) => parseResponse<{ destinations: number[] }>(response))
        .then((body) => body.destinations)
}

export function makeMove(gameId: string, move: MoveRequest): Promise<GameStateDTO> {
    return fetch(`${API_BASE_URL}/games/${gameId}/move`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(move),
    }).then((response) => parseResponse<GameStateDTO>(response))
}

export function undoMove(gameId: string): Promise<GameStateDTO> {
    return fetch(`${API_BASE_URL}/games/${gameId}/undo`, { method: 'POST' }).then((response) =>
        parseResponse<GameStateDTO>(response),
    )
}
