```rust
use super::chess_piece::ChessPiece;
use std::fmt;

pub enum Color {
    White,
    Black,
}

pub enum Square {
    Empty,
    Occupied(Box<dyn ChessPiece>),
}

pub struct Chessboard {
    squares: [[Square; 8]; 8],
}

impl Chessboard {
    pub fn new() -> Chessboard {
        Chessboard {
            squares: [[Square::Empty; 8]; 8],
        }
    }

    pub fn place_piece(&mut self, piece: Box<dyn ChessPiece>, x: usize, y: usize) {
        self.squares[x][y] = Square::Occupied(piece);
    }

    pub fn get_square(&self, x: usize, y: usize) -> &Square {
        &self.squares[x][y]
    }
}

impl fmt::Display for Chessboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.squares {
            for square in row {
                match square {
                    Square::Empty => write!(f, ". "),
                    Square::Occupied(piece) => write!(f, "{} ", piece.ascii_char()),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
```