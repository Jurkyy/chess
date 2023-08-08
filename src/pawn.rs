```rust
use crate::piece::Piece;
use crate::board::Board;

pub struct Pawn {
    piece: Piece,
    first_move: bool,
}

impl Pawn {
    pub fn new(piece: Piece) -> Self {
        Pawn {
            piece,
            first_move: true,
        }
    }

    pub fn move_piece(&mut self, board: &mut Board, new_position: (usize, usize)) -> Result<(), &'static str> {
        let current_position = self.piece.position;
        let valid_moves = self.valid_moves();

        if valid_moves.contains(&new_position) {
            self.piece.position = new_position;
            self.first_move = false;
            Ok(())
        } else {
            Err("Invalid move for Pawn")
        }
    }

    pub fn valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = self.piece.position;

        if self.first_move {
            moves.push((x + 2, y));
        }

        moves.push((x + 1, y));

        if y > 0 {
            moves.push((x + 1, y - 1));
        }

        if y < 7 {
            moves.push((x + 1, y + 1));
        }

        moves
    }

    pub fn promote(&mut self, new_piece: Piece) {
        if self.piece.position.0 == 7 {
            self.piece = new_piece;
        }
    }
}
```