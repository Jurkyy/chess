```rust
use crate::chess::piece::{Piece, Color};
use crate::chess::position::Position;

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        for i in 0..8 {
            squares[1][i] = Some(Piece::new(Color::White));
            squares[6][i] = Some(Piece::new(Color::Black));
        }

        squares[0][0] = Some(Piece::new(Color::White));
        squares[0][7] = Some(Piece::new(Color::White));
        squares[7][0] = Some(Piece::new(Color::Black));
        squares[7][7] = Some(Piece::new(Color::Black));

        Board { squares }
    }

    pub fn get_piece(&self, position: &Position) -> Option<&Piece> {
        self.squares[position.y as usize][position.x as usize].as_ref()
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) -> Result<(), &'static str> {
        let piece = self.squares[from.y as usize][from.x as usize].take();

        match piece {
            Some(piece) => {
                if piece.is_valid_move(from, to) {
                    self.squares[to.y as usize][to.x as usize] = Some(piece);
                    Ok(())
                } else {
                    Err("Invalid move")
                }
            }
            None => Err("No piece at the given position"),
        }
    }
}
```