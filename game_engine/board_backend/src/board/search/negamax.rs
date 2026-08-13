use crate::board::position::Position;
use crate::board::types::Move;
use crate::board::zobric::{Bound, TTEntry};

const INFINITY: i32 = 2_000_000;
const CHECK_MATE: i32 = 1_000_000;

impl Position{
    pub fn negamax(&mut self, depth: u32, ply: u32, mut alpha: i32, beta: i32) -> i32{
        let original_alpha = alpha;
        let hash = self.board.zobrian_hash;

        // A position recurring anywhere in the real game so far, or anywhere earlier in this
        // same search branch, is treated as a draw and cut off immediately -- before the TT
        // lookup, since a cached score from a context where this same hash wasn't a repetition
        // would otherwise be reused here incorrectly. This must never be written back into
        // the transposition table below either, since "is this a repetition" depends on the
        // search path, not on the hash alone.
        let repetition_count = self.position_history.iter().filter(|&&h| h == hash).count()
            + self.search_path_hashes.iter().filter(|&&h| h == hash).count();
        if repetition_count >= 2 {
            return 0;
        }

        // Transposition table lookup: a hit is only usable if it was searched at least as
        // deep as we need right now, and its bound is conclusive for the current alpha/beta
        // window. This runs before move generation so a hit skips that work entirely.
        // Terminal (checkmate/stalemate) and depth==0 leaves are never stored below (neither
        // explores any moves, so there's no best_move to record), so a hit here can never be
        // mistaken for one of those -- it's always a real, previously-completed search.
        if let Some(entry) = self.transposition_table.get(&hash) {
            if entry.depth >= depth {
                match entry.bound {
                    Bound::Exact => return entry.score,
                    Bound::LowerBound if entry.score >= beta => return entry.score,
                    Bound::UpperBound if entry.score <= alpha => return entry.score,
                    _ => {}
                }
            }
        }

        let mut legal_moves = self.board.all_legal_moves();

        // Terminal node: no legal moves means checkmate or stalemate. Checked before the
        // depth == 0 case so a mate discovered exactly at the horizon is still recognized
        // as one, instead of falling back to a plain material evaluate(). ply is the number
        // of half-moves played since find_best_move's root call, so a mate found sooner
        // (smaller ply) scores closer to -CHECK_MATE, and one found later scores less
        // negative -- preferring faster mates once this bubbles back up through negation.
        if legal_moves.is_empty() {
            return if self.board.is_in_check(self.board.side_to_move) {
                -CHECK_MATE + ply as i32
            } else {
                0
            };
        }

        if depth == 0 {
            return self.board.evaluate(self.moves_counter);
        }

        self.order_moves(&mut legal_moves);

        let mut best = -INFINITY;
        let mut best_move: Option<Move> = None;

        for m in legal_moves {
            self.make_move(m);
            self.search_path_hashes.push(self.board.zobrian_hash);
            let score = -self.negamax(depth - 1, ply + 1, -beta, -alpha);
            self.search_path_hashes.pop();
            self.undo_move();

            if score > best {
                best = score;
                best_move = Some(m);
            }
            if best > alpha {
                alpha = best;
            }
            if alpha >= beta {
                break;
            }
        }

        // Classify which kind of bound this result represents, then store it for reuse by
        // any future call that transposes into this same position.
        let bound = if best >= beta {
            Bound::LowerBound
        } else if best <= original_alpha {
            Bound::UpperBound
        } else {
            Bound::Exact
        };

        self.transposition_table.insert(hash, TTEntry {
            depth,
            bound,
            score: best,
            best_move: best_move.expect("legal_moves was non-empty, so the loop ran at least once"),
        });

        best
    }

    pub fn find_best_move(&mut self, depth: u32) -> Option<Move> {
        self.search_path_hashes.clear();

        let mut legal_moves: Vec<Move> = self.board.all_legal_moves();

        if legal_moves.is_empty() {
            return None;
        }

        // Apply ordering
        self.order_moves(&mut legal_moves);


        let mut alpha = -INFINITY;
        let beta = INFINITY;
        let mut best_move = None;
        let mut best_score = -INFINITY;

        for m in legal_moves {
            self.make_move(m);
            self.search_path_hashes.push(self.board.zobrian_hash);
            // We negate because the player to move after our move is the opponent: negamax
            // scores from whoever's turn it is at that node, so the opponent's best result
            // is exactly as good for them as it is bad for us -- negating converts it back
            // to our own perspective. ply starts at 1 here since one move has been played.
            // depth.saturating_sub(1) avoids underflowing depth (a u32) if depth == 0.
            let score = -self.negamax(depth.saturating_sub(1), 1, -beta, -alpha);
            self.search_path_hashes.pop();
            self.undo_move();

            if score > best_score {
                best_score = score;
                best_move = Some(m);
            }
            if best_score > alpha {
                alpha = best_score;
            }

            // CHECK_MATE - 1 is the score of a true mate-in-1 (the fastest possible mate
            // from the root) -- nothing examined afterward could ever beat it, so it's safe
            // to stop here. Any *slower* confirmed mate must NOT break early, since a later
            // root move could still deliver a faster one that best_score would prefer.
            if best_score == CHECK_MATE - 1 {
                break;
            }
        }

        best_move
    }


    pub fn order_moves(&self, moves: &mut [Move]) {
        // Sort_unstable_by_key allow us to filter a Vec or Slice giving it a closure (in this case the score of the capture)
        // It is more faster than the sort_by_key because in that case it matters the order of elements wtih the same result in the closure
        // so a heap copy must be used (all of this internally in the method). We negate the score_move because the sort_unstable_by_key order from smaller to bigger so
        // the most valuable captures will be last (contrary of what we want), so we negate the result
        moves.sort_unstable_by_key(|mv| -self.board.score_move(mv, self.moves_counter));
    }
}
