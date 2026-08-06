use std::{collections::HashMap, sync::{Arc, Mutex}};

use board_backend::board::{Color, position::Position};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

// One played ply, with a snapshot of the board right after it -- lets the
// frontend scrub through past positions by index, without replaying moves
// or duplicating the engine's move logic client-side.
pub struct MoveRecord {
    pub ply: u32,
    pub color: Color,
    pub notation: String,
    pub squares: Vec<u8>,
}

pub struct Game {
    pub position: Position,
    pub user_color: Color,
    pub depth: u32,
    pub move_history: Vec<MoveRecord>,
}

impl AppState {
    pub fn new() -> Self{
        Self { 
            games: Arc::new(Mutex::new(HashMap::new())) 
        }
    }

    pub fn games(&self) -> Arc<Mutex<HashMap<Uuid, Game>>>{
        self.games.clone()
    }
}
