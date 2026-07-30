use std::collections::HashMap;

use crate::board::zobric::TTEntry;

use super::state::Board;
use super::make_move::Action;


pub struct Position{
    pub board: Board,
    pub history: Vec<Action>,
    pub transposition_table: HashMap<u64, TTEntry>,
}