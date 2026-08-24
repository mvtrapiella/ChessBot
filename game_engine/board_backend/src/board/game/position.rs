use core::panic;
use std::collections::HashMap;
use std::io::stdin;
use std::println;

use crate::board::Color::{Black, White};
use crate::board::types::{
    Move, Color, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK, WHITE_BISHOP, WHITE_KNIGHT,
    BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, BLACK_BISHOP, BLACK_KNIGHT,
};
use crate::board::zobric::TTEntry;
use crate::board::negamax::SearchLimit;

use crate::board::state::Board;
use super::make_move::Action;


pub struct Position{
    pub board: Board,
    pub history: Vec<Action>,
    pub transposition_table: HashMap<u64, TTEntry>,
    // Hashes of positions actually reached during the real game (pushed once per real
    // move in game_play/user_make_move) -- kept separate from transposition_table, which
    // gets overwritten with unrelated hypothetical positions explored during search, and
    // wouldn't give an accurate repetition count.
    pub position_history: Vec<u64>,
    pub moves_counter: u32,
    pub search_path_hashes: Vec<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DrawReason {
    Stalemate,
    FiftyMoveRule,
    InsufficientMaterial,
    ThreefoldRepetition,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameStatus {
    InProgress,
    WhiteWon,
    BlackWon,
    Drawn(DrawReason),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveError {
    IllegalMove,
}



impl Position{
    pub fn game_play(&mut self, depth: u32){
        let user_color = self.color_selection();
        self.position_history.push(self.board.zobrian_hash);
        self.board.print_board(user_color);

        while self.game_status() == GameStatus::InProgress{
            self.user_make_move(user_color, depth);
            self.record_real_move();
            self.board.print_board(user_color);
        }

        Self::print_game_over(self.game_status());
    }

    pub fn record_real_move(&mut self) {
        self.position_history.push(self.board.zobrian_hash);
        self.moves_counter += 1;
    }

    fn print_game_over(status: GameStatus) {
        match status {
            GameStatus::Drawn(DrawReason::Stalemate) => println!("Drawn by stalemate"),
            GameStatus::Drawn(DrawReason::FiftyMoveRule) => println!("Drawn by the 50-move rule"),
            GameStatus::Drawn(DrawReason::InsufficientMaterial) => println!("Drawn by insufficient material"),
            GameStatus::Drawn(DrawReason::ThreefoldRepetition) => println!("Drawn by threefold repetition"),
            GameStatus::WhiteWon => println!("White won"),
            GameStatus::BlackWon => println!("Black won"),
            GameStatus::InProgress => {}
        }
    }

    // Apply a move given in UCI-style square notation ("e2e4", "e7e8q"). Only accepts
    // moves that are actually legal in this position; the turn is switched internally.
    pub fn apply_move_str(&mut self, input: &str) -> Result<(), MoveError> {
        let mv = self.parse_legal_move(input).ok_or(MoveError::IllegalMove)?;
        self.make_move(mv);
        Ok(())
    }

    fn user_make_move(&mut self, user_color: Color, depth: u32){
        if user_color == self.board.side_to_move{
            let user_move = self.obtain_user_move();
            // Make the move of the user, the turn is changed internally
            self.make_move(user_move);
        }
        else{
            let machine_move = self.find_best_move(SearchLimit::Depth(depth)).expect("There should be a move to do");

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

    pub fn game_status(&self) -> GameStatus{
        if self.board.all_legal_moves().is_empty() {
            if !self.board.is_in_check(self.board.side_to_move) {
                return GameStatus::Drawn(DrawReason::Stalemate);
            }

            return match self.board.side_to_move {
                Color::White => GameStatus::BlackWon,
                Color::Black => GameStatus::WhiteWon,
            };
        }

        if self.board.halfmove_clock == 100 {
            return GameStatus::Drawn(DrawReason::FiftyMoveRule);
        }

        if self.is_insufficient_material() {
            return GameStatus::Drawn(DrawReason::InsufficientMaterial);
        }

        if self.is_threefold_repetition() {
            return GameStatus::Drawn(DrawReason::ThreefoldRepetition);
        }

        GameStatus::InProgress
    }

    // No pawns/rooks/queens anywhere, and at most one minor piece (knight or bishop)
    // total between both sides -- covers K v K, K+B v K and K+N v K, the cases where
    // forced checkmate is genuinely impossible. Deliberately conservative: positions
    // like K+N+N v K or K+B v K+B aren't flagged, since those aren't always dead draws.
    fn is_insufficient_material(&self) -> bool {
        let bb = &self.board.piece_bitboards;

        let pawns_or_majors = bb[WHITE_PAWN as usize - 1] | bb[WHITE_ROOK as usize - 1] | bb[WHITE_QUEEN as usize - 1]
            | bb[BLACK_PAWN as usize - 1] | bb[BLACK_ROOK as usize - 1] | bb[BLACK_QUEEN as usize - 1];

        if pawns_or_majors != 0 {
            return false;
        }

        let minor_count = bb[WHITE_KNIGHT as usize - 1].count_ones()
            + bb[WHITE_BISHOP as usize - 1].count_ones()
            + bb[BLACK_KNIGHT as usize - 1].count_ones()
            + bb[BLACK_BISHOP as usize - 1].count_ones();

        minor_count <= 1
    }

    // Counts how many times the current position's hash has occurred in the real game
    // (position_history), not the search tree -- three occurrences (including the
    // current one) is a draw by repetition.
    fn is_threefold_repetition(&self) -> bool {
        let current_hash = self.board.zobrian_hash;

        self.position_history.iter().filter(|&&hash| hash == current_hash).count() >= 3
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

    // Inverse of square_from_str: a square index into its 2-character algebraic form ("e2").
    pub fn square_to_str(square: u8) -> String {
        let file = (b'a' + (square % 8)) as char;
        let rank = (square / 8) + 1;
        format!("{}{}", file, rank)
    }

    // Parses a 2-character algebraic square ("e2") into a square index. Public so
    // callers outside this crate (e.g. the web server) don't need to duplicate this.
    pub fn square_from_str(square: &str) -> Option<u8> {
        let mut chars = square.chars();
        let file = chars.next()?;
        let rank = chars.next()?;

        if chars.next().is_some() {
            return None;
        }

        Self::square_from_chars(file, rank)
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