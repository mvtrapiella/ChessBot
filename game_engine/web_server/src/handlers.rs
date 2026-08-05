use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use board_backend::board::{position::GameStatus::InProgress, state::Board, types::Move};
use board_backend::board::types::{Color, NO_SQUARE};
use board_backend::board::position::Position;

use crate::dto::{CreateGameRequest, GameStateDTO, MoveRequest};
use crate::error::ApiError;
use crate::state::{AppState, Game};

pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<CreateGameRequest>,
) -> Result<Json<GameStateDTO>, ApiError> {
    if payload.depth == 0 {
        return Err(ApiError::BadRequest("depth must be a positive integer".to_string()));
    }

    let mut board = Board {
        squares: [0; 64],
        piece_bitboards: [0; 12],
        white_pieces: 0,
        black_pieces: 0,
        all_pieces: 0,
        side_to_move: Color::White,
        castling_rights: 15,
        en_passant_square: NO_SQUARE,
        halfmove_clock: 0,
        zobrian_hash: 0,
    };

    board.initialize_board();
    board.update_bitboards();

    let position = Position {
        board,
        history: Vec::new(),
        transposition_table: HashMap::new(),
        position_history: Vec::new(),
    };

    let game_id = Uuid::new_v4();
    let user_color: Color = payload.user_color.into();
    let response = GameStateDTO::from_position(game_id, &position, user_color);

    let game = Game {
        position,
        user_color,
        depth: payload.depth,
    };

    state.games().lock().unwrap().insert(game_id, game);

    Ok(Json(response))
}

pub async fn get_game(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<GameStateDTO>, ApiError> {
    let games = state.games();
    let games = games.lock().unwrap();

    let game = games.get(&game_id).ok_or(ApiError::NotFound)?;

    Ok(Json(GameStateDTO::from_position(game_id, &game.position, game.user_color)))
}

pub async fn make_move(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
    Json(payload): Json<MoveRequest>,
) -> Result<Json<GameStateDTO>, ApiError> {

    let games = state.games();
    let mut games = games.lock().unwrap();

    let game = games.get_mut(&game_id).ok_or(ApiError::NotFound)?;

    if game.position.board.side_to_move != game.user_color {
        return Err(ApiError::BadRequest("It's not your turn".to_string()));
    }

    let promotion = payload.promotion.map(|c| c.to_string()).unwrap_or_default();
    let move_str = format!("{}{}{}", payload.origin, payload.destination, promotion);

    game.position
        .apply_move_str(&move_str)
        .map_err(|_| ApiError::BadRequest("Illegal move".to_string()))?;

    game.position.position_history.push(game.position.board.zobrian_hash);

    if game.position.game_status() == InProgress {
        let bot_mv: Move = game.position.find_best_move(game.depth).expect("The game has not ended so the bot should make a move");

        game.position.make_move(bot_mv);
        game.position.position_history.push(game.position.board.zobrian_hash);
    } 

    Ok(Json(GameStateDTO::from_position(game_id, &game.position, game.user_color)))
}
