use crate::board::Board;

use super::PartialPiece;

pub struct Knight {}

impl PartialPiece for Knight {
    fn validate_move(from: (u8, u8), to: (u8, u8), _board: &Board) -> bool {
        let (delta_x, delta_y) = (from.0.abs_diff(to.0), from.1.abs_diff(to.1));

        (delta_x == 1 && delta_y == 2) || (delta_y == 1 && delta_x == 2)
    }
}