use serde::{Deserialize, Serialize};
use uuid::Uuid;

use board_backend::board::position::{DrawReason, GameStatus, Position};
use board_backend::board::types::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ColorDTO {
    White,
    Black,
}

// Obtain the color from the DTO
impl From<ColorDTO> for Color {
    fn from(color: ColorDTO) -> Self {
        match color {
            ColorDTO::White => Color::White,
            ColorDTO::Black => Color::Black,
        }
    }
}

// Obtain the ColorDTO from the Color
impl From<Color> for ColorDTO {
    fn from(color: Color) -> Self {
        match color {
            Color::White => ColorDTO::White,
            Color::Black => ColorDTO::Black,
        }
    }
}

// Enum that represent the game states (the same as the one in the board_backend crate)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStatusDTO {
    InProgress,
    WhiteWon,
    BlackWon,
    Drawn,
}

// Enum that represent the possible drawn reasons ((the same as the one in the board_backend crate)
// Used for telling the user the reason of the drawn
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DrawReasonDTO {
    Stalemate,
    FiftyMoveRule,
    InsufficientMaterial,
    ThreefoldRepetition,
}

// Struct that stores the user game request info 
// -> the depth the user wants the bot to operate and the color the user wants to be
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameRequest {
    pub user_color: ColorDTO,
    pub depth: u32,
}

// Struct that stores the move made by the user
#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub origin: String,
    pub destination: String,
    pub promotion: Option<char>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateDTO {
    pub game_id: Uuid,
    pub squares: Vec<u8>,
    pub side_to_move: ColorDTO,
    pub user_color: ColorDTO,
    pub status: GameStatusDTO,
    pub draw_reason: Option<DrawReasonDTO>,
}

impl GameStateDTO {
    // Check the actual position state and returns a GameStateDTO with the actual game state
    pub fn from_position(game_id: Uuid, position: &Position, user_color: Color) -> Self {
        let (status, draw_reason) = match position.game_status() {
            GameStatus::InProgress => (GameStatusDTO::InProgress, None),
            GameStatus::WhiteWon => (GameStatusDTO::WhiteWon, None),
            GameStatus::BlackWon => (GameStatusDTO::BlackWon, None),
            GameStatus::Drawn(reason) => (
                GameStatusDTO::Drawn,
                Some(match reason {
                    DrawReason::Stalemate => DrawReasonDTO::Stalemate,
                    DrawReason::FiftyMoveRule => DrawReasonDTO::FiftyMoveRule,
                    DrawReason::InsufficientMaterial => DrawReasonDTO::InsufficientMaterial,
                    DrawReason::ThreefoldRepetition => DrawReasonDTO::ThreefoldRepetition,
                }),
            ),
        };

        GameStateDTO {
            game_id,
            squares: position.board.squares.to_vec(),
            side_to_move: position.board.side_to_move.into(),
            user_color: user_color.into(),
            status,
            draw_reason,
        }
    }
}
