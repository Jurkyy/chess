use std::io::{self, Write};

pub fn handle_input() -> Result<(usize, usize, usize, usize), &'static str> {
    let mut input = String::new();
    print!("Enter your move (format: x1 y1 x2 y2): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.len() {
        4 => {
            let parse_result: Result<Vec<usize>, _> = parts.iter().map(|s| s.parse()).collect();

            match parse_result {
                Ok(parsed_values) => {
                    let (x1, y1, x2, y2) = if let [x1, y1, x2, y2] = parsed_values.as_slice() {
                        (x1, y1, x2, y2)
                    } else {
                        todo!()
                    };
                    if x1 <= &7 && y1 <= &7 && x2 <= &7 && y2 <= &7 {
                        Ok((*x1, *y1, *x2, *y2))
                    } else {
                        Err("Invalid input. Coordinates must be between 0 and 7.")
                    }
                }
                Err(_) => {
                    Err("Invalid input. Please enter four valid numbers separated by spaces.")
                }
            }
        }
        _ => Err("Invalid input. Please enter four numbers separated by spaces."),
    }
}
