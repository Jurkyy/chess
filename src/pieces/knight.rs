use crate::pieces::piece::Piece;
use crate::board::Board;

pub struct Knight {
    pub position: (usize, usize),
    pub color: String,
}

impl Piece for Knight {
    fn new(position: (usize, usize), color: String) -> Self {
        Knight { position, color }
    }

    fn get_color(&self) -> String {
        self.color.clone()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn can_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let start = self.get_position();
        let possible_moves = [
            (start.0 + 2, start.1 + 1),
            (start.0 + 2, start.1 - 1),
            (start.0 - 2, start.1 + 1),
            (start.0 - 2, start.1 - 1),
            (start.0 + 1, start.1 + 2),
            (start.0 + 1, start.1 - 2),
            (start.0 - 1, start.1 + 2),
            (start.0 - 1, start.1 - 2),
        ];

        if possible_moves.contains(&end) {
            if let Some(piece) = board.get_piece(end) {
                return piece.get_color() != self.get_color();
            }
            return true;
        }
        false
    }
}