```rust
use crate::chess::piece::Piece;
use crate::chess::game::GameStatus;
use crate::chess::PIECES;

pub const BOARD_SIZE: usize = 8;

pub struct Board {
    pub pieces: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
    pub status: GameStatus,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = [[None; BOARD_SIZE]; BOARD_SIZE];
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if i == 0 || i == 1 || i == 6 || i == 7 {
                    pieces[i][j] = Some(Piece::new(PIECES[i / 6], i / 6, (i, j)));
                }
            }
        }
        Board {
            pieces,
            status: GameStatus::Ongoing,
        }
    }

    pub fn get_piece(&self, position: (usize, usize)) -> Option<&Piece> {
        self.pieces[position.0][position.1].as_ref()
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), &'static str> {
        let piece = self.pieces[from.0][from.1].take();
        match piece {
            Some(p) => {
                if p.is_valid_move(to) {
                    self.pieces[to.0][to.1] = Some(p);
                    Ok(())
                } else {
                    Err("Invalid move")
                }
            },
            None => Err("No piece at the given position"),
        }
    }
}
```