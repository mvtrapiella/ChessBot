use core::panic;
use std::collections::HashMap;
use std::io::stdin;
use std::println;

use crate::board::Color::{Black, White};
use crate::board::position::GameState::{BlackLost, Drawn, InProgress, WhiteLost};
use crate::board::types::{
    Move, Color, WHITE_QUEEN, WHITE_ROOK, WHITE_BISHOP, WHITE_KNIGHT,
    BLACK_QUEEN, BLACK_ROOK, BLACK_BISHOP, BLACK_KNIGHT,
};
use crate::board::zobric::TTEntry;

use super::state::Board;
use super::make_move::Action;


pub struct Position{
    pub board: Board,
    pub history: Vec<Action>,
    pub transposition_table: HashMap<u64, TTEntry>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameState{
    Drawn,
    WhiteLost,
    BlackLost,
    InProgress,
}



impl Position{
    pub fn game_play(&mut self, depth: u32){
        let user_color = self.color_selection();
        self.board.print_board();

        while self.check_game_state() == InProgress{
            self.user_make_move(user_color, depth);
            self.board.print_board();
        }
    }

    fn user_make_move(&mut self, user_color: Color, depth: u32){
        if user_color == self.board.side_to_move{
            let user_move = self.obtain_user_move();
            // Make the move of the user, the turn is changed internally
            self.make_move(user_move);
        }
        else{
            let machine_move = self.find_best_move(depth).expect("There should be a move to do");

            self.make_move(machine_move);
        }
    }

    fn color_selection(&self) -> Color{
        let mut color_str: String = String::new();
        let white_str: String = "WHITE".to_owned();
        let black_str: String = "BLACK".to_owned();
            
        while color_str.to_uppercase().trim() != white_str && color_str.to_uppercase().trim() != black_str{
            println!("Enter your color: WHITE or BLACK");

            stdin()
            .read_line(&mut color_str)
            .expect("Error reading the line");
        }
        
         match color_str.trim().to_uppercase().as_str() {
            "WHITE" => White,
            "BLACK" => Black,
            _ => panic!("The color: {} is not a player color on chess", color_str),
        }
    }

    fn check_game_state(&self) -> GameState{
        if self.board.side_to_move == Color::White && self.board.all_legal_moves().is_empty(){
            println!("Black won");
            WhiteLost
        }
        else if self.board.side_to_move == Color::Black && self.board.all_legal_moves().is_empty() {
            println!("White won");
            BlackLost
        }
        else if self.board.halfmove_clock == 100 {
            println!("Drawn");
            Drawn
        }
        else {
            InProgress
        }
    }

    fn obtain_user_move(&self) -> Move{
        let mut mv: Option<Move> = None;

        while mv.is_none() {
            println!("Enter your move:");

            let mut mv_string: String = String::new();
            stdin()
            .read_line(&mut mv_string)
            .expect("Error reading the line");
            mv = self.parse_legal_move(&mv_string);

            if mv.is_none() {
                println!("Illegal move");
            }
        }

        mv.expect("There must be a move")
    }

    // Parses the string, then only accepts it if it's one of the actual legal moves in this
    // position -- rejects well-formed but illegal input (e.g. moving through a piece,
    // leaving the king in check) the same way move_from_string alone can't.
    fn parse_legal_move(&self, input: &str) -> Option<Move> {
        let mv = self.move_from_string(input)?;
        let legal_moves = self.board.all_legal_moves();

        if legal_moves.contains(&mv) {
            Some(mv)
        } else {
            None
        }
    }

    // Parses UCI-style square notation ("e2e4", "e7e8q") into a Move. Doesn't check
    // legality -- that's all_legal_moves's job, this only rejects malformed strings.
    fn move_from_string(&self, input: &str) -> Option<Move> {
        let input = input.trim();
        let chars: Vec<char> = input.chars().collect();

        if chars.len() != 4 && chars.len() != 5 {
            return None;
        }

        let origin = Self::square_from_chars(chars[0], chars[1])?;
        let destination = Self::square_from_chars(chars[2], chars[3])?;

        let promotion = if chars.len() == 5 {
            Some(self.promotion_piece_from_char(chars[4])?)
        } else {
            None
        };

        Some(Move { origin, destination, promotion })
    }

    fn square_from_chars(file: char, rank: char) -> Option<u8> {
        let file = file.to_ascii_lowercase() as i8 - 'a' as i8;
        let rank = rank as i8 - '1' as i8;

        if !(0..8).contains(&file) || !(0..8).contains(&rank) {
            return None;
        }

        Some((rank * 8 + file) as u8)
    }

    fn promotion_piece_from_char(&self, c: char) -> Option<u8> {
        let is_white = self.board.side_to_move == Color::White;

        match c.to_ascii_lowercase() {
            'q' => Some(if is_white { WHITE_QUEEN } else { BLACK_QUEEN }),
            'r' => Some(if is_white { WHITE_ROOK } else { BLACK_ROOK }),
            'b' => Some(if is_white { WHITE_BISHOP } else { BLACK_BISHOP }),
            'n' => Some(if is_white { WHITE_KNIGHT } else { BLACK_KNIGHT }),
            _ => None,
        }
    }
}