```rust
use crate::chess::piece::Piece;
use crate::chess::color::Color;

pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [[None; 8]; 8];

        for i in 0..8 {
            squares[1][i] = Some(Piece::new(Color::White, "Pawn"));
            squares[6][i] = Some(Piece::new(Color::Black, "Pawn"));
        }

        squares[0][0] = Some(Piece::new(Color::White, "Rook"));
        squares[0][7] = Some(Piece::new(Color::White, "Rook"));
        squares[7][0] = Some(Piece::new(Color::Black, "Rook"));
        squares[7][7] = Some(Piece::new(Color::Black, "Rook"));

        squares[0][1] = Some(Piece::new(Color::White, "Knight"));
        squares[0][6] = Some(Piece::new(Color::White, "Knight"));
        squares[7][1] = Some(Piece::new(Color::Black, "Knight"));
        squares[7][6] = Some(Piece::new(Color::Black, "Knight"));

        squares[0][2] = Some(Piece::new(Color::White, "Bishop"));
        squares[0][5] = Some(Piece::new(Color::White, "Bishop"));
        squares[7][2] = Some(Piece::new(Color::Black, "Bishop"));
        squares[7][5] = Some(Piece::new(Color::Black, "Bishop"));

        squares[0][3] = Some(Piece::new(Color::White, "Queen"));
        squares[7][3] = Some(Piece::new(Color::Black, "Queen"));

        squares[0][4] = Some(Piece::new(Color::White, "King"));
        squares[7][4] = Some(Piece::new(Color::Black, "King"));

        Self { squares }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<&Piece> {
        self.squares[x][y].as_ref()
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        let piece = self.squares[from.0][from.1].take();
        self.squares[to.0][to.1] = piece;
    }
}
```