use crate::board::{ Board };

use super::PartialPiece;

pub struct King {}

impl PartialPiece for King {
    fn validate_move(from: (u8, u8), to: (u8, u8), _board: &Board) -> bool {
        // TODO: Castling
        return from.0.abs_diff(to.0) <= 1 && from.1.abs_diff(to.1) <= 1;
    }
}