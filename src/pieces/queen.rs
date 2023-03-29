use crate::board::Board;

use super::{ PartialPiece, bishop::Bishop, rook::Rook };

pub struct Queen {}

impl PartialPiece for Queen {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        return Bishop::validate_move(from, to, board) || Rook::validate_move(from, to, board);
    }
}