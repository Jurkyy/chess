import unittest
from chess_game.chess_game import ChessGame
from chess_game.chess_board import ChessBoard
from chess_game.chess_piece import ChessPiece
from chess_game.chess_player import ChessPlayer
from chess_game.castling import Castling

class TestCastling(unittest.TestCase):

    def setUp(self):
        self.game = ChessGame()
        self.board = ChessBoard()
        self.player1 = ChessPlayer('white')
        self.player2 = ChessPlayer('black')
        self.castling = Castling()

    def test_castling_move(self):
        # Set up a scenario where castling is possible
        self.board.clear_board_except_king_and_rook(self.player1)
        self.board.clear_board_except_king_and_rook(self.player2)

        # Test castling for player1
        self.assertTrue(self.castling.is_castling_possible(self.player1))
        self.castling.perform_castling(self.player1)
        self.assertFalse(self.castling.is_castling_possible(self.player1))

        # Test castling for player2
        self.assertTrue(self.castling.is_castling_possible(self.player2))
        self.castling.perform_castling(self.player2)
        self.assertFalse(self.castling.is_castling_possible(self.player2))

    def test_castling_not_possible(self):
        # Set up a scenario where castling is not possible
        self.board.setup_board()

        # Test castling for player1
        self.assertFalse(self.castling.is_castling_possible(self.player1))

        # Test castling for player2
        self.assertFalse(self.castling.is_castling_possible(self.player2))

if __name__ == '__main__':
    unittest.main()