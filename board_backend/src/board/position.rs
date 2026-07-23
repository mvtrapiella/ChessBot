use super::state::Board;
use super::make_move::Action;


pub struct Position{
    pub board: Board,
    pub history: Vec<Action>,
}