```rust
use std::io::{self, Write};

pub fn handle_input() -> Result<(usize, usize, usize, usize), &'static str> {
    let mut input = String::new();
    print!("Enter your move (format: x1 y1 x2 y2): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 4 {
        return Err("Invalid input. Please enter four numbers separated by spaces.");
    }

    let x1 = parts[0].parse::<usize>().unwrap();
    let y1 = parts[1].parse::<usize>().unwrap();
    let x2 = parts[2].parse::<usize>().unwrap();
    let y2 = parts[3].parse::<usize>().unwrap();

    if x1 > 7 || y1 > 7 || x2 > 7 || y2 > 7 {
        return Err("Invalid input. Coordinates must be between 0 and 7.");
    }

    Ok((x1, y1, x2, y2))
}
```