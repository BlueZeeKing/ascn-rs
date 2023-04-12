use std::{ fmt::Display, error };

use crate::{ board::{ Board, position::BoardPosition }, pieces::{ PieceType, Player, Piece } };

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let fields = fen.split(" ").collect::<Vec<_>>();

        if fields.len() != 6 {
            return Err(Error::NotEnoughFields);
        }

        let mut board = Self::blank_board();

        board.set_to_move(if fields[1] == "w" { Player::White } else { Player::Black });

        let rows = fields[0].split("/").collect::<Vec<_>>();

        if rows.len() != 8 {
            return Err(Error::NotEnoughRanks);
        }

        let mut rank_pos: u8 = 8;

        for row in rows {
            let mut file_pos: u8 = 1;

            for piece in row.chars() {
                let piece_type = match piece.to_ascii_lowercase() {
                    'p' => PieceType::Pawn,
                    'r' => PieceType::Rook,
                    'q' => PieceType::Queen,
                    'k' => PieceType::King,
                    'n' => PieceType::Knight,
                    'b' => PieceType::Bishop,
                    str => {
                        match str.to_digit(10) {
                            Some(number) => {
                                file_pos += number as u8;
                                continue;
                            }
                            None => {
                                return Err(Error::InvalidPiece);
                            }
                        }
                    }
                };

                let player = if piece.is_uppercase() { Player::White } else { Player::Black };

                board.set_square(
                    &BoardPosition::new(file_pos as u8, rank_pos),
                    &Some(Piece(piece_type, player))
                );

                file_pos += 1;
            }

            if file_pos != 9 {
                return Err(Error::NotEnoughFiles);
            }

            rank_pos -= 1;
        }

        Ok(board)
    }
}

#[derive(Debug)]
pub enum Error {
    NotEnoughFields,
    NotEnoughRanks,
    NotEnoughFiles,
    InvalidPiece,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::NotEnoughFields => "Not enough fields in the fen string",
            Error::NotEnoughRanks => "Not enough ranks could be found in the fen string",
            Error::NotEnoughFiles => "Not enough files were found in a rank",
            Error::InvalidPiece => "Invalid piece character",
        })
    }
}