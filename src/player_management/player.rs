```rust
pub enum Color {
    White,
    Black,
}

pub struct Player {
    name: String,
    color: Color,
}

impl Player {
    pub fn new(name: String, color: Color) -> Player {
        Player { name, color }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}
```