use crate::board::{ Board };

use super::PartialPiece;

pub struct King {}

impl PartialPiece for King {
    fn validate_move(from: (u8, u8), to: (u8, u8), _board: &Board) -> bool {
        // TODO: Castling
        return from.0.abs_diff(to.0) <= 1 && from.1.abs_diff(to.1) <= 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, position::BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece, king::King },
    };

    #[test]
    fn chess_move() {
        let mut board = Board::blank_board();

        board.set_square(&BoardPosition::new(5, 1), &Some(Piece(PieceType::King, Player::White)));

        assert_eq!(King::validate_move((5, 1), (5, 2), &board), true);
        assert_eq!(King::validate_move((5, 1), (6, 2), &board), true);
        assert_eq!(King::validate_move((5, 1), (6, 1), &board), true);
        assert_eq!(King::validate_move((5, 1), (5, 5), &board), false);
        assert_eq!(King::validate_move((5, 1), (8, 1), &board), false);
    }
}