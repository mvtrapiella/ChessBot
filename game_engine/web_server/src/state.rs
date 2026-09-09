use std::{collections::HashMap, sync::{Arc, Mutex}, time::Instant};

use board_backend::board::{Color, position::Position};
use board_backend::board::negamax::SearchLimit;
use uuid::Uuid;

// Bounds how much memory a flood of created games (bot or otherwise) can take on the VM.
// Once full, creating a game evicts whichever existing game has gone longest without a
// move, rather than rejecting the request outright -- so the service stays usable for new
// players even while under pressure, instead of getting permanently wedged once the cap is
// first hit.
pub const MAX_GAMES: usize = 500;

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
    pub search_limit: SearchLimit,
    pub move_history: Vec<MoveRecord>,
    pub last_active: Instant,
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

    // Called right before inserting a newly created game. Evicts the single
    // longest-idle game if the store is already at capacity, so the map never grows past
    // MAX_GAMES regardless of how many games get created.
    pub fn evict_oldest_if_full(games: &mut HashMap<Uuid, Game>) {
        if games.len() < MAX_GAMES {
            return;
        }
        if let Some(&oldest_id) = games
            .iter()
            .min_by_key(|(_, game)| game.last_active)
            .map(|(id, _)| id)
        {
            games.remove(&oldest_id);
        }
    }
}
