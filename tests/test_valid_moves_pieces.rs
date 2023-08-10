use std::rc::Rc;

use chess::{
    game_components::{
        bishop::Bishop,
        chessboard::{Chessboard, Color},
        king::King,
        knight::Knight,
        pawn::Pawn,
        queen::Queen,
        rook::Rook,
    },
    user_interface::print_chessboard::{print_chessboard, print_valid_moves},
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
    assert_eq!(valid_moves, vec![(1, 2), (1, 3)]); // Modify this based on your implementation
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
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (0, 4),
        (0, 5),
        (0, 6),
        (0, 7),
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
    let expected_moves = vec![(3, 1), (4, 2), (5, 3), (6, 4), (7, 5), (1, 1)];
    assert_eq!(valid_moves, expected_moves);
}

#[test]
fn test_king_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();
    chessboard.place_piece(Rc::new(King::new(Color::White, false)), 4, 0);

    let king = chessboard.get_piece(4, 0).unwrap();

    print_chessboard(&chessboard);

    let valid_moves = king.valid_moves((4, 0), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    let expected_moves = vec![(5, 0), (3, 0), (4, 1), (5, 1), (3, 1)];
    assert_eq!(valid_moves, expected_moves);
}

#[test]
fn test_queen_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();

    chessboard.place_piece(Rc::new(Queen::new(Color::White)), 5, 0);

    let queen = chessboard.get_piece(5, 0).unwrap();

    print_valid_moves(&*queen, (5, 0), &chessboard);

    let valid_moves = queen.valid_moves((5, 0), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    let expected_moves = vec![
        (6, 0),
        (7, 0),
        (4, 0),
        (3, 0),
        (2, 0),
        (1, 0),
        (0, 0),
        (5, 1),
        (5, 2),
        (5, 3),
        (5, 4),
        (5, 5),
        (5, 6),
        (5, 7),
        (6, 1),
        (7, 2),
        (4, 1),
        (3, 2),
        (2, 3),
        (1, 4),
    ];
    assert_eq!(valid_moves, expected_moves);
}

#[test]
fn test_knight_valid_moves() {
    // Initialize the chessboard with the desired positions
    let mut chessboard = Chessboard::new();
    chessboard.place_piece(Rc::new(Knight::new(Color::White)), 4, 4);

    let knight = chessboard.get_piece(4, 4).unwrap();

    print_valid_moves(&*knight, (4, 4), &chessboard);

    let valid_moves = knight.valid_moves((4, 4), &chessboard);

    // Compare valid_moves with the expected valid moves for that position
    let expected_moves = vec![
        (2, 3),
        (2, 5),
        (3, 2),
        (3, 6),
        (5, 2),
        (5, 6),
        (6, 3),
        (6, 5),
    ];
    assert_eq!(valid_moves, expected_moves);
}
