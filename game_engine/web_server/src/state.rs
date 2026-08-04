use std::{collections::HashMap, sync::{Arc, Mutex}};

use board_backend::board::{Color, position::Position};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    games: Arc<Mutex<HashMap<Uuid, Game>>>,
}

pub struct Game {
    pub position: Position,
    pub user_color: Color,
    pub depth: u32,
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
