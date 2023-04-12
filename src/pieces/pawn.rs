use crate::board::{ position::BoardPosition, Board };

use super::{ PartialPiece, Player };

pub struct Pawn {}

impl PartialPiece for Pawn {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        // TODO: En passant
        let piece = board
            .get_square(&BoardPosition::from(from))
            .as_ref()
            .expect("Could not find pawn");

        let should_move_forward = piece.1 == Player::White; // In the positive direction

        let (delta_x, delta_y) = (from.0.abs_diff(to.0), from.1.abs_diff(to.1));

        if (should_move_forward && from.1 > to.1) || (!should_move_forward && from.1 < to.1) {
            return false;
        }

        if delta_x == 1 && delta_y == 1 && board.get_square(&BoardPosition::from(to)).is_some() {
            true
        } else if
            delta_x == 0 &&
            delta_y == 1 &&
            board.get_square(&BoardPosition::from(to)).is_none()
        {
            true
        } else if
            delta_x == 0 &&
            delta_y == 2 &&
            ((from.1 == 2 && piece.1 == Player::White) ||
                (from.1 == 7 && piece.1 == Player::Black)) &&
            board.get_square(&BoardPosition::from(to)).is_none() &&
            board
                .get_square(
                    &BoardPosition::new(from.0, if should_move_forward {
                        from.1 + 1
                    } else {
                        from.1 - 1
                    })
                )
                .is_none()
        {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, position::BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece, pawn::Pawn },
    };

    #[test]
    fn empty_1() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Pawn, Player::White)));

        assert_eq!(Pawn::validate_move((1, 1), (1, 2), &board), true);
    }

    #[test]
    fn empty_2() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 7), &Some(Piece(PieceType::Pawn, Player::Black)));

        assert_eq!(Pawn::validate_move((1, 7), (1, 5), &board), true);
    }

    #[test]
    fn forward_2() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 3), &Some(Piece(PieceType::Pawn, Player::White)));

        assert_eq!(Pawn::validate_move((1, 3), (1, 5), &board), false);
    }

    #[test]
    fn in_the_way_1() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Pawn, Player::White)));
        board.set_square(&BoardPosition::new(1, 2), &Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Pawn::validate_move((1, 1), (1, 2), &board), false);
    }

    #[test]
    fn in_the_way_2() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 2), &Some(Piece(PieceType::Pawn, Player::White)));
        board.set_square(&BoardPosition::new(1, 3), &Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Pawn::validate_move((1, 2), (1, 4), &board), false);
    }

    #[test]
    fn capture() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Pawn, Player::White)));
        board.set_square(&BoardPosition::new(2, 2), &Some(Piece(PieceType::Rook, Player::Black)));

        assert_eq!(Pawn::validate_move((1, 1), (2, 2), &board), true);
    }

    #[test]
    fn horizontal() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(1, 1), &Some(Piece(PieceType::Pawn, Player::White)));

        assert_eq!(Pawn::validate_move((1, 1), (2, 1), &board), false);
    }
}