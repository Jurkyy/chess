use std::io::{self, Write};

pub fn handle_input() -> Result<(usize, usize, usize, usize), &'static str> {
    let mut input = String::new();
    print!("Enter your move (format: e.g., a2 a3): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts.len() {
        2 => {
            let (x1, y1) = parse_coordinates(parts[0]);
            let (x2, y2) = parse_coordinates(parts[1]);

            if x1 <= 7 && y1 <= 7 && x2 <= 7 && y2 <= 7 {
                Ok((x1, y1, x2, y2))
            } else {
                Err("Invalid input. Coordinates must be between 0 and 7.")
            }
        }
        _ => {
            Err("Invalid input. Please enter two coordinates in the format 'file rank file rank'.")
        }
    }
}

fn parse_coordinates(input: &str) -> (usize, usize) {
    let chars: Vec<char> = input.chars().collect();
    let x = chars[0] as usize - 'a' as usize;
    let y = chars[1..]
        .iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
        - 1;
    (x, y)
}
