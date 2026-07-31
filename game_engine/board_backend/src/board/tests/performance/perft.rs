use crate::board::position::Position;

impl Position{
    pub fn perft(&mut self, depth: usize) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut nodes = 0;
        let moves = self.board.all_legal_moves();

        for mv in moves {
            self.make_move(mv);
            nodes += self.perft(depth - 1);
            self.undo_move();
        }

        nodes
    }
}