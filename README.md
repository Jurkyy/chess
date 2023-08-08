# Chess Game in Rust

This is a simple implementation of a chess game in Rust. It includes all standard chess rules including castling and pawn promotion.

## Structure

The project is structured into several files and modules:

- `main.rs`: The main entry point of the application.
- `board.rs`: Contains the `Board` struct and its associated methods.
- `pieces/`: A module containing the implementation of all the chess pieces.
  - `pawn.rs`: Contains the `Pawn` struct and its associated methods.
  - `knight.rs`: Contains the `Knight` struct and its associated methods.
  - `bishop.rs`: Contains the `Bishop` struct and its associated methods.
  - `rook.rs`: Contains the `Rook` struct and its associated methods.
  - `queen.rs`: Contains the `Queen` struct and its associated methods.
  - `king.rs`: Contains the `King` struct and its associated methods.
  - `piece.rs`: Contains the `Piece` trait which is implemented by all the chess pieces.
- `tests/`: A module containing all the tests for the application.
  - `board_tests.rs`: Contains tests for the `Board` struct.
  - `pieces_tests.rs`: Contains tests for all the chess pieces.

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

## Running the Tests

To run the tests, use the following command:

```bash
cargo test
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)