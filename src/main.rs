```rust
use std::io::{self, Write};
mod board;
mod pieces;

fn main() {
    let mut game_board = board::Board::new();
    game_loop(&mut game_board);
}

fn game_loop(game_board: &mut board::Board) {
    loop {
        game_board.display();
        let move = get_move();
        game_board.make_move(move);
        if game_board.checkmate() {
            println!("Checkmate!");
            break;
        }
        if game_board.stalemate() {
            println!("Stalemate!");
            break;
        }
    }
}

fn get_move() -> board::Move {
    print!("Enter your move: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    parse_move(&input.trim())
}

fn parse_move(input: &str) -> board::Move {
    let split: Vec<&str> = input.split_whitespace().collect();
    let start = parse_position(split[0]);
    let end = parse_position(split[1]);
    board::Move { start, end }
}

fn parse_position(input: &str) -> board::Position {
    let x = input.chars().nth(0).unwrap() as usize - 'a' as usize;
    let y = input.chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
    board::Position { x, y }
}
```