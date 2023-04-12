use crate::board::Board;

use super::PartialPiece;

pub struct Knight {}

impl PartialPiece for Knight {
    fn validate_move(from: (u8, u8), to: (u8, u8), _board: &Board) -> bool {
        let (delta_x, delta_y) = (from.0.abs_diff(to.0), from.1.abs_diff(to.1));

        (delta_x == 1 && delta_y == 2) || (delta_y == 1 && delta_x == 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, position::BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece, knight::Knight },
    };

    #[test]
    fn chess_move() {
        let mut board = Board::new([[None; 8]; 8]);

        board.set_square(&BoardPosition::new(1, 2), &Some(Piece(PieceType::Knight, Player::White)));
        board.set_square(&BoardPosition::new(3, 3), &Some(Piece(PieceType::Knight, Player::Black)));
        board.set_square(&BoardPosition::new(5, 2), &Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Knight::validate_move((2, 1), (3, 3), &board), true);
        assert_eq!(Knight::validate_move((3, 3), (5, 2), &board), true);
        assert_eq!(Knight::validate_move((5, 2), (6, 4), &board), true);
    }
}