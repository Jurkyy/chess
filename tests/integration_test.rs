use std::rc::Rc;

use chess::{
    game_components::{
        bishop::Bishop,
        chessboard::{Chessboard, Color},
        pawn::Pawn,
        rook::Rook,
    },
    player_management::player::Player,
};

#[test]
fn test_pawn_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();

    chessboard.place_piece(Rc::new(Pawn::new(Color::White, false)), 1, 1);

    // Get the pawn at the desired position
    let pawn = chessboard.get_piece(1, 1).unwrap();
    // Validate the pawn's valid moves
    let valid_moves = pawn.valid_moves((1, 1), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    assert_eq!(valid_moves, vec![(1, 2), (1, 2)]); // Modify this based on your implementation
}

#[test]
fn test_rook_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();
    chessboard.place_piece(Rc::new(Rook::new(Color::White, false)), 0, 0);

    // Get the rook at the desired position
    let rook = chessboard.get_piece(0, 0).unwrap();

    // Validate the rook's valid moves
    let valid_moves = rook.valid_moves((0, 0), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    let expected_moves = vec![
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
    ];
    assert_eq!(valid_moves, expected_moves);
    // Modify this based on your implementation
}

#[test]
fn test_bishop_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();
    chessboard.place_piece(Rc::new(Bishop::new(Color::White)), 2, 0);

    // Get the bishop at the desired position
    let bishop = chessboard.get_piece(2, 0).unwrap();
    // Validate the bishop's valid moves
    let valid_moves = bishop.valid_moves((2, 0), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    let expected_moves = vec![(3, 1), (4, 2), (5, 3), (6, 4), (7, 5)];
    assert_eq!(valid_moves, expected_moves);
}
