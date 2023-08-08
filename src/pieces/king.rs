use crate::board::Board;
use crate::pieces::piece::Piece;

pub struct King {
    pub position: (usize, usize),
    pub color: bool,
    pub has_moved: bool,
}

impl Piece for King {
    fn new(color: bool, position: (usize, usize)) -> Self {
        King {
            position,
            color,
            has_moved: false,
        }
    }

    fn get_color(&self) -> bool {
        self.color
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn can_move(&self, board: &Board, mov: &Move) -> bool {
        let dx = (self.position.0 as i32 - mov.destination.0 as i32).abs();
        let dy = (self.position.1 as i32 - mov.destination.1 as i32).abs();

        if dx > 1 || dy > 1 {
            return false;
        }

        if let Some(piece) = board.get_piece(mov.destination) {
            if piece.get_color() == self.color {
                return false;
            }
        }

        true
    }

    fn can_castle(&self, board: &Board, mov: &Move) -> bool {
        if self.has_moved {
            return false;
        }

        let dx = self.position.0 as i32 - mov.destination.0 as i32;

        if dx.abs() != 2 {
            return false;
        }

        let rook_position = if dx > 0 {
            (0, self.position.1)
        } else {
            (7, self.position.1)
        };

        if let Some(rook) = board.get_piece(rook_position) {
            if rook.get_color() == self.color && rook.can_castle(board, mov) {
                return true;
            }
        }

        false
    }
}
