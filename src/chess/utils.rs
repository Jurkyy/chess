```rust
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_valid(&self) -> bool {
        self.x < 8 && self.y < 8
    }
}

pub fn is_castling_move(start: &Position, end: &Position) -> bool {
    let x_distance = (start.x as i32 - end.x as i32).abs();
    let y_distance = (start.y as i32 - end.y as i32).abs();
    x_distance == 2 && y_distance == 0
}
```