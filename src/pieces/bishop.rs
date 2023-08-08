```rust
use crate::board::Board;
use crate::pieces::Piece;

pub struct Bishop {
    pub position: (usize, usize),
    pub color: String,
}

impl Bishop {
    pub fn new(position: (usize, usize), color: String) -> Bishop {
        Bishop { position, color }
    }

    pub fn valid_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let dx = (end.0 as i32 - self.position.0 as i32).abs();
        let dy = (end.1 as i32 - self.position.1 as i32).abs();

        if dx == dy && !board.path_is_blocked(self.position, end) {
            true
        } else {
            false
        }
    }
}

impl Piece for Bishop {
    fn position(&self) -> (usize, usize) {
        self.position
    }

    fn color(&self) -> String {
        self.color.clone()
    }

    fn move_piece(&mut self, end: (usize, usize)) {
        self.position = end;
    }
}
```