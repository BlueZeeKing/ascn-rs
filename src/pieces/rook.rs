use crate::board::{ position::BoardPosition, Board };

use super::PartialPiece;

pub struct Rook {}

impl PartialPiece for Rook {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        let (delta_x, delta_y) = (from.0.abs_diff(to.0), from.1.abs_diff(to.1));

        if delta_x != 0 && delta_y != 0 {
            return false;
        }

        if delta_y == 0 {
            for x in get_path_iter(from.0, to.0) {
                if board.get_square(&BoardPosition::new(x, from.1)).is_some() {
                    return false;
                }
            }
        } else {
            for y in get_path_iter(from.1, to.1) {
                if board.get_square(&BoardPosition::new(from.0, y)).is_some() {
                    return false;
                }
            }
        }

        true
    }
}

fn get_path_iter(first: u8, second: u8) -> Vec<u8> {
    if first < second { (first + 1..second).collect() } else { (second + 1..first).rev().collect() }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, position::BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece, rook::Rook },
    };

    #[test]
    fn empty_x() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 2), &Some(Piece(PieceType::Rook, Player::White)));

        assert_eq!(Rook::validate_move((1, 2), (5, 2), &board), true);
    }

    #[test]
    fn empty_y() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Rook, Player::White)));

        assert_eq!(Rook::validate_move((1, 1), (1, 8), &board), true);
    }

    #[test]
    fn in_the_way_1() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Rook, Player::White)));
        board.set_square(&BoardPosition::new(1, 2), &Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Rook::validate_move((1, 1), (1, 3), &board), false);
    }

    #[test]
    fn in_the_way_2() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Rook, Player::White)));
        board.set_square(&BoardPosition::new(2, 1), &Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Rook::validate_move((1, 1), (3, 1), &board), false);
    }

    #[test]
    fn capture() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Rook, Player::White)));
        board.set_square(&BoardPosition::new(1, 8), &Some(Piece(PieceType::Rook, Player::Black)));

        assert_eq!(Rook::validate_move((1, 1), (1, 8), &board), true);
    }

    #[test]
    fn diag() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Rook, Player::White)));

        assert_eq!(Rook::validate_move((1, 1), (8, 8), &board), false);
    }
}