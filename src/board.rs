pub mod iterator;
pub mod position;

use std::fmt::Display;

use crate::{ pieces::{ Piece, Player, PieceType }, fen::STARTING_FEN };

use self::position::BoardPosition;

const RANK_SEPARATOR: &str = "\n---+---+---+---+---+---+---+---\n";

const BLANK_BOARD: [[Option<Piece>; 8]; 8] = [
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
];

#[derive(Clone, Debug)]
pub struct Board {
    board: [[Option<Piece>; 8]; 8],
    to_move: Player,
}

impl Board {
    pub fn new(board: [[Option<Piece>; 8]; 8]) -> Self {
        Self { board, to_move: Player::White }
    }

    pub fn blank_board() -> Self {
        Self {
            board: BLANK_BOARD,
            to_move: Player::White,
        }
    }

    pub fn starting_board() -> Self {
        Self::from_fen(STARTING_FEN).expect("Bundled starting board could not be parsed")
    }

    pub fn set_to_move(&mut self, player: Player) {
        self.to_move = player;
    }

    pub fn get_to_move(&self) -> &Player {
        &self.to_move
    }

    pub fn set_square(&mut self, pos: &BoardPosition, value: &Option<Piece>) {
        let (x, y) = pos.get_index();
        self.board[y as usize][x as usize] = value.clone();
    }

    pub fn get_square(&self, pos: &BoardPosition) -> &Option<Piece> {
        let (x, y) = pos.get_index();
        &self.board[y as usize][x as usize]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board
            .iter()
            .map(|rank|
                rank
                    .clone()
                    .map(|piece| format!(" {} ", piece_to_char(&piece)))
                    .join("|")
            )
            .rev()
            .collect::<Vec<_>>()
            .join(RANK_SEPARATOR);
        write!(f, "{}", board)
    }
}

fn piece_to_char(possible_piece: &Option<Piece>) -> char {
    if let Some(piece) = possible_piece {
        return match piece.0 {
            PieceType::Queen => if piece.1 == Player::Black { '♛' } else { '♕' }
            PieceType::King => if piece.1 == Player::Black { '♚' } else { '♔' }
            PieceType::Rook => if piece.1 == Player::Black { '♜' } else { '♖' }
            PieceType::Bishop => if piece.1 == Player::Black { '♝' } else { '♗' }
            PieceType::Pawn => if piece.1 == Player::Black { '♟' } else { '♙' }
            PieceType::Knight => if piece.1 == Player::Black { '♞' } else { '♘' }
        };
    }

    ' '
}

#[derive(Clone)]
pub struct Move {
    pub from: BoardPosition,
    pub to: BoardPosition,
}