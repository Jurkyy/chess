use crate::board::Board;
use crate::pieces::Piece;

pub struct Pawn {
    position: (usize, usize),
    color: String,
    first_move: bool,
}

impl Pawn {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Pawn {
            position,
            color,
            first_move: true,
        }
    }

    pub fn move_piece(&mut self, new_position: (usize, usize), board: &mut Board) {
        if self.valid_move(new_position, board) {
            self.position = new_position;
            self.first_move = false;
        }
    }

    pub fn valid_move(&self, new_position: (usize, usize), board: &Board) -> bool {
        let (x, y) = self.position;
        let (new_x, new_y) = new_position;

        if self.first_move && new_y == y + 2 && new_x == x {
            return true;
        }

        if new_y == y + 1 && new_x == x {
            return true;
        }

        if new_y == y + 1 && (new_x == x + 1 || new_x == x - 1) {
            if let Some(piece) = board.get_piece(new_position) {
                if piece.color != self.color {
                    return true;
                }
            }
        }

        false
    }

    pub fn promote(&mut self, new_piece: Piece) {
        if self.position.1 == 7 {
            *self = Pawn::new(self.position, self.color);
        }
    }
}
