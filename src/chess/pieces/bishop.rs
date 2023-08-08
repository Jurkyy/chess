```rust
use super::mod::Piece;
use crate::chess::board::Board;
use crate::chess::utils::Position;

pub struct Bishop {
    position: Position,
    white: bool,
}

impl Bishop {
    pub fn new(position: Position, white: bool) -> Self {
        Bishop { position, white }
    }
}

impl Piece for Bishop {
    fn is_valid_move(&self, board: &Board, new_position: Position) -> bool {
        let dx = (self.position.x - new_position.x).abs();
        let dy = (self.position.y - new_position.y).abs();
        dx == dy && board.is_path_clear(&self.position, &new_position)
    }

    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn is_white(&self) -> bool {
        self.white
    }
}
```