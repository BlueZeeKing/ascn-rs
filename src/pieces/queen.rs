use crate::board::Board;

use super::{ PartialPiece, bishop::Bishop, rook::Rook };

pub struct Queen {}

impl PartialPiece for Queen {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        return Bishop::validate_move(from, to, board) || Rook::validate_move(from, to, board);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece, queen::Queen },
    };

    #[test]
    fn chess_move() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(4, 1), Some(Piece(PieceType::Queen, Player::White)));

        assert_eq!(Queen::validate_move((4, 1), (4, 8), &board), true);
        assert_eq!(Queen::validate_move((4, 1), (6, 3), &board), true);
        assert_eq!(Queen::validate_move((4, 1), (1, 4), &board), true);
        assert_eq!(Queen::validate_move((4, 1), (2, 2), &board), false);
        assert_eq!(Queen::validate_move((4, 1), (8, 2), &board), false);
    }
}