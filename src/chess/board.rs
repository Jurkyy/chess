```rust
use super::pieces::{Piece, King, Rook};
use super::utils::Position;

pub struct Board {
    pub pieces: Vec<Box<dyn Piece>>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: Vec::new(),
        }
    }

    pub fn add_piece(&mut self, piece: Box<dyn Piece>) {
        self.pieces.push(piece);
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), &'static str> {
        let piece_index = self.pieces.iter().position(|piece| piece.get_position() == from);

        match piece_index {
            Some(index) => {
                self.pieces[index].set_position(to);
                Ok(())
            },
            None => Err("No piece at the given position"),
        }
    }

    pub fn can_castle(&self, king: &King, rook: &Rook) -> bool {
        if king.has_moved() || rook.has_moved() {
            return false;
        }

        let king_position = king.get_position();
        let rook_position = rook.get_position();

        if king_position.y != rook_position.y {
            return false;
        }

        let min_x = king_position.x.min(rook_position.x);
        let max_x = king_position.x.max(rook_position.x);

        for x in (min_x + 1)..max_x {
            if self.pieces.iter().any(|piece| piece.get_position() == Position { x, y: king_position.y }) {
                return false;
            }
        }

        true
    }
}
```