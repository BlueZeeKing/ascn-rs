use crate::board::{ Move, Board };

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub trait PartialPiece {
    fn validate_and_execute_move(chess_move: &Move, board: &Board) -> (bool, Board) {
        let is_valid = Self::validate_move(chess_move.from.tuple(), chess_move.to.tuple(), board);

        return (is_valid, Self::execute_move(is_valid, board, chess_move));
    }

    fn validate_move(from: (u8, u8), to: (u8, u8), board: &Board) -> bool;

    fn execute_move(is_valid: bool, board: &Board, chess_move: &Move) -> Board {
        let mut new_board = board.clone();

        let piece = board.get_square(chess_move.from).as_ref().expect("Could not find piece");

        if is_valid {
            new_board.set_square(chess_move.from, None);
            new_board.set_square(chess_move.to, Some(piece.clone()));
        }

        new_board
    }
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Queen,
    King,
    Rook,
    Bishop,
    Pawn,
    Knight,
}

impl PieceType {
    pub fn validate_move(&self, chess_move: &Move, board: &Board) -> (bool, Board) {
        match self {
            PieceType::Queen => queen::Queen::validate_and_execute_move(&chess_move, &board),
            PieceType::King => king::King::validate_and_execute_move(&chess_move, &board),
            PieceType::Rook => rook::Rook::validate_and_execute_move(&chess_move, &board),
            PieceType::Bishop => bishop::Bishop::validate_and_execute_move(&chess_move, &board),
            PieceType::Pawn => pawn::Pawn::validate_and_execute_move(&chess_move, &board),
            PieceType::Knight => knight::Knight::validate_and_execute_move(&chess_move, &board),
        }
    }
}

#[derive(Clone, PartialEq, Copy)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece(PieceType, Player);