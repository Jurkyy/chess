```rust
use crate::piece::Piece;
use crate::pawn::Pawn;
use crate::knight::Knight;
use crate::bishop::Bishop;
use crate::rook::Rook;
use crate::queen::Queen;
use crate::king::King;

pub struct Board {
    pub squares: [[Option<Box<dyn Piece>>; 8]; 8],
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
}
```