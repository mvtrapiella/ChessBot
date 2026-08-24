use std::collections::HashMap;
use std::time::Duration;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use board_backend::board::{position::GameStatus::InProgress, state::Board, types::Move};
use board_backend::board::types::{Color, NO_SQUARE};
use board_backend::board::position::Position;
use board_backend::board::negamax::SearchLimit;

use crate::dto::{CreateGameRequest, GameStateDTO, LegalMovesDTO, MoveRequest};
use crate::error::ApiError;
use crate::notation::{describe_move, piece_letter};
use crate::state::{AppState, Game, MoveRecord};

const MAX_DIFFICULTY_TIME_BUDGET: Duration = Duration::from_secs(5);

pub async fn create_game(
    State(state): State<AppState>,
    Json(payload): Json<CreateGameRequest>,
) -> Result<Json<GameStateDTO>, ApiError> {
    let search_limit = if payload.max_difficulty {
        SearchLimit::TimeBudget(MAX_DIFFICULTY_TIME_BUDGET)
    } else {
        if payload.depth == 0 {
            return Err(ApiError::BadRequest("depth must be a positive integer".to_string()));
        }
        SearchLimit::Depth(payload.depth)
    };

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

    let mut position = Position {
        board,
        history: Vec::new(),
        transposition_table: HashMap::new(),
        position_history: Vec::new(),
        moves_counter: 0,
        search_path_hashes: Vec::new(),
        nodes: 0,
        deadline: None,
        search_aborted: false,
    };
    position.position_history.push(position.board.zobrian_hash);

    let game_id = Uuid::new_v4();
    let user_color: Color = payload.user_color.into();
    let mut move_history: Vec<MoveRecord> = Vec::new();

    if position.board.side_to_move != user_color {
        let squares_before = position.board.squares;
        let mover_color = position.board.side_to_move;

        let opening_move = position
            .find_best_move(search_limit)
            .expect("The game has not ended so the bot should make a move");
        position.make_move(opening_move);
        position.record_real_move();

        let promotion_letter = opening_move
            .promotion
            .map(|p| piece_letter(p).chars().next().expect("promotion piece always has a letter"));

        move_history.push(MoveRecord {
            ply: 1,
            color: mover_color,
            notation: describe_move(
                squares_before,
                opening_move.origin,
                opening_move.destination,
                promotion_letter,
                &position,
            ),
            squares: position.board.squares.to_vec(),
        });
    }

    let response = GameStateDTO::from_position(game_id, &position, user_color, &move_history);

    let game = Game {
        position,
        user_color,
        search_limit,
        move_history,
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

    Ok(Json(GameStateDTO::from_position(
        game_id,
        &game.position,
        game.user_color,
        &game.move_history,
    )))
}

pub async fn legal_moves(
    State(state): State<AppState>,
    Path((game_id, square)): Path<(Uuid, String)>,
) -> Result<Json<LegalMovesDTO>, ApiError> {
    let games = state.games();
    let games = games.lock().unwrap();

    let game = games.get(&game_id).ok_or(ApiError::NotFound)?;

    let origin = Position::square_from_str(&square)
        .ok_or_else(|| ApiError::BadRequest(format!("Invalid square: {}", square)))?;

    let destinations = game
        .position
        .board
        .all_legal_moves()
        .into_iter()
        .filter(|mv| mv.origin == origin)
        .map(|mv| mv.destination)
        .collect();

    Ok(Json(LegalMovesDTO { destinations }))
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

    let origin = Position::square_from_str(&payload.origin)
        .ok_or_else(|| ApiError::BadRequest("Invalid origin square".to_string()))?;
    let destination = Position::square_from_str(&payload.destination)
        .ok_or_else(|| ApiError::BadRequest("Invalid destination square".to_string()))?;

    let promotion_suffix = payload.promotion.map(|c| c.to_string()).unwrap_or_default();
    let move_str = format!("{}{}{}", payload.origin, payload.destination, promotion_suffix);

    let squares_before_user_move = game.position.board.squares;
    let user_move_color = game.position.board.side_to_move;

    game.position
        .apply_move_str(&move_str)
        .map_err(|_| ApiError::BadRequest("Illegal move".to_string()))?;

    game.position.record_real_move();

    let user_ply = game.move_history.len() as u32 + 1;
    game.move_history.push(MoveRecord {
        ply: user_ply,
        color: user_move_color,
        notation: describe_move(
            squares_before_user_move,
            origin,
            destination,
            payload.promotion.map(|c| c.to_ascii_uppercase()),
            &game.position,
        ),
        squares: game.position.board.squares.to_vec(),
    });

    if game.position.game_status() == InProgress {
        let squares_before_bot_move = game.position.board.squares;
        let bot_color = game.position.board.side_to_move;

        let bot_mv: Move = game.position.find_best_move(game.search_limit).expect("The game has not ended so the bot should make a move");
        game.position.make_move(bot_mv);
        game.position.record_real_move();

        let promotion_letter = bot_mv
            .promotion
            .map(|p| piece_letter(p).chars().next().expect("promotion piece always has a letter"));

        let bot_ply = game.move_history.len() as u32 + 1;
        game.move_history.push(MoveRecord {
            ply: bot_ply,
            color: bot_color,
            notation: describe_move(
                squares_before_bot_move,
                bot_mv.origin,
                bot_mv.destination,
                promotion_letter,
                &game.position,
            ),
            squares: game.position.board.squares.to_vec(),
        });
    }

    Ok(Json(GameStateDTO::from_position(
        game_id,
        &game.position,
        game.user_color,
        &game.move_history,
    )))
}

// Real undo, not the frontend's visual scrubbing: rewinds both the bot's
// last reply and the user's move before it, so it's the user's turn again
// at the position they were actually at.
pub async fn undo_move(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<GameStateDTO>, ApiError> {
    let games = state.games();
    let mut games = games.lock().unwrap();

    let game = games.get_mut(&game_id).ok_or(ApiError::NotFound)?;

    if game.move_history.len() < 2 {
        return Err(ApiError::BadRequest("Nothing to undo".to_string()));
    }

    game.position.undo_move();
    game.position.undo_move();

    let position_history_len = game.position.position_history.len();
    game.position.position_history.truncate(position_history_len - 2);
    game.position.moves_counter = game.position.moves_counter.saturating_sub(2);

    let move_history_len = game.move_history.len();
    game.move_history.truncate(move_history_len - 2);

    Ok(Json(GameStateDTO::from_position(
        game_id,
        &game.position,
        game.user_color,
        &game.move_history,
    )))
}
