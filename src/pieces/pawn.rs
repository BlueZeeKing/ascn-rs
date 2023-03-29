use crate::board::{ BoardPosition, Board };

use super::{ PartialPiece, Player };

pub struct Pawn {}

impl PartialPiece for Pawn {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        // TODO: En passant
        let piece = board
            .get_square(BoardPosition::from(from))
            .as_ref()
            .expect("Could not find pawn");

        let should_move_forward = piece.1 == Player::White; // In the positive direction

        let (delta_x, delta_y) = (from.0.abs_diff(to.0), from.1.abs_diff(to.1));

        if (should_move_forward && from.1 > to.1) || (!should_move_forward && from.1 < to.1) {
            return false;
        }

        if delta_x == 1 && delta_y == 1 && board.get_square(BoardPosition::from(to)).is_some() {
            true
        } else if
            delta_x == 0 &&
            delta_y == 1 &&
            board.get_square(BoardPosition::from(to)).is_none()
        {
            board.get_square(BoardPosition::from(to)).is_some()
        } else if
            delta_x == 0 &&
            delta_y == 2 &&
            board.get_square(BoardPosition::from(to)).is_none() &&
            board
                .get_square(
                    BoardPosition::new(from.0, if should_move_forward {
                        from.1 + 1
                    } else {
                        from.1 - 1
                    })
                )
                .is_none()
        {
            board.get_square(BoardPosition::from(to)).is_some()
        } else {
            false
        }
    }
}