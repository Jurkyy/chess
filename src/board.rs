```rust
use crate::pieces::{Piece, Pawn, Knight, Bishop, Rook, Queen, King};

pub struct Board {
    squares: [[Option<Box<dyn Piece>>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        for i in 0..8 {
            squares[1][i] = Some(Box::new(Pawn::new(true)));
            squares[6][i] = Some(Box::new(Pawn::new(false)));
        }

        squares[0][0] = Some(Box::new(Rook::new(true)));
        squares[0][7] = Some(Box::new(Rook::new(true)));
        squares[7][0] = Some(Box::new(Rook::new(false)));
        squares[7][7] = Some(Box::new(Rook::new(false)));

        squares[0][1] = Some(Box::new(Knight::new(true)));
        squares[0][6] = Some(Box::new(Knight::new(true)));
        squares[7][1] = Some(Box::new(Knight::new(false)));
        squares[7][6] = Some(Box::new(Knight::new(false)));

        squares[0][2] = Some(Box::new(Bishop::new(true)));
        squares[0][5] = Some(Box::new(Bishop::new(true)));
        squares[7][2] = Some(Box::new(Bishop::new(false)));
        squares[7][5] = Some(Box::new(Bishop::new(false)));

        squares[0][3] = Some(Box::new(Queen::new(true)));
        squares[7][3] = Some(Box::new(Queen::new(false)));

        squares[0][4] = Some(Box::new(King::new(true)));
        squares[7][4] = Some(Box::new(King::new(false)));

        Self { squares }
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        let piece = match self.squares[from.0][from.1].take() {
            Some(piece) => piece,
            None => return Err("No piece at the given position"),
        };

        if piece.valid_move(from, to) {
            if let Some(target) = &self.squares[to.0][to.1] {
                if target.is_white() == piece.is_white() {
                    return Err("Cannot capture own piece");
                }
            }

            self.squares[to.0][to.1] = Some(piece);
            Ok(())
        } else {
            self.squares[from.0][from.1] = Some(piece);
            Err("Invalid move for piece")
        }
    }

    pub fn promote_pawn(&mut self, position: (usize, usize), new_piece: Box<dyn Piece>) -> Result<(), &'static str> {
        let piece = match self.squares[position.0][position.1].take() {
            Some(piece) => piece,
            None => return Err("No piece at the given position"),
        };

        if piece.as_any().is::<Pawn>() && (position.0 == 0 || position.0 == 7) {
            self.squares[position.0][position.1] = Some(new_piece);
            Ok(())
        } else {
            self.squares[position.0][position.1] = Some(piece);
            Err("Can only promote a pawn at the end of the board")
        }
    }
}
```