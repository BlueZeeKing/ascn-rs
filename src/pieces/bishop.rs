use crate::board::{ Board, BoardPosition };

use super::PartialPiece;

pub struct Bishop {}

impl PartialPiece for Bishop {
    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool {
        let path: Vec<u8> = if from.0 < to.0 {
            (from.0 + 1..to.0).collect()
        } else {
            (to.0 + 1..from.0).rev().collect()
        };

        for x in path {
            let y = if from.1 < to.1 {
                from.1 + x.abs_diff(from.0)
            } else {
                from.1 - x.abs_diff(from.0)
            };

            if board.get_square(BoardPosition::from((x, y))).is_some() {
                return false;
            }
        }

        return from.0.abs_diff(to.0) == from.1.abs_diff(to.1);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::{ Board, BoardPosition },
        pieces::{ PieceType, Piece, Player, PartialPiece },
    };

    use super::Bishop;

    #[test]
    fn empty_diag() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 1), Some(Piece(PieceType::Bishop, Player::White)));

        assert_eq!(Bishop::validate_move((1, 1), (8, 8), &board), true);
    }

    #[test]
    fn empty_diag_2() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 8), Some(Piece(PieceType::Bishop, Player::White)));

        assert_eq!(Bishop::validate_move((1, 8), (8, 1), &board), true);
    }

    #[test]
    fn in_the_way_1() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 1), Some(Piece(PieceType::Bishop, Player::White)));
        board.set_square(BoardPosition::new(2, 2), Some(Piece(PieceType::Knight, Player::White)));

        assert_eq!(Bishop::validate_move((1, 1), (8, 8), &board), false);
    }

    #[test]
    fn in_the_way_2() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 1), Some(Piece(PieceType::Bishop, Player::White)));
        board.set_square(BoardPosition::new(7, 7), Some(Piece(PieceType::Rook, Player::Black)));

        assert_eq!(Bishop::validate_move((1, 1), (8, 8), &board), false);
    }

    #[test]
    fn capture() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 1), Some(Piece(PieceType::Bishop, Player::White)));
        board.set_square(BoardPosition::new(8, 8), Some(Piece(PieceType::Rook, Player::Black)));

        assert_eq!(Bishop::validate_move((1, 1), (8, 8), &board), true);
    }

    #[test]
    fn vert() {
        let mut board = Board {
            board: [[None; 8]; 8],
        };

        board.set_square(BoardPosition::new(1, 1), Some(Piece(PieceType::Bishop, Player::White)));

        assert_eq!(Bishop::validate_move((1, 1), (1, 8), &board), false);
    }
}