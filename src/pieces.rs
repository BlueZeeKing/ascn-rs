use crate::board::{ Move, Board, position::BoardPosition, iterator::BoardIterator };

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

        let piece = board.get_square(&chess_move.from).as_ref().expect("Could not find piece");

        if is_valid {
            new_board.set_square(&chess_move.from, &None);
            new_board.set_square(&chess_move.to, &Some(piece.clone()));
        }

        new_board
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    Queen,
    King,
    Rook,
    Bishop,
    Pawn,
    Knight,
}

impl Piece {
    pub fn validate_and_execute_move(chess_move: &Move, board: &Board) -> (bool, Board) {
        let piece;

        if let Some(inner_piece) = board.get_square(&chess_move.from) {
            piece = inner_piece;
        } else {
            return (false, board.clone());
        }

        let (is_valid, new_board) = match piece.0 {
            PieceType::Queen => queen::Queen::validate_and_execute_move(&chess_move, &board),
            PieceType::King => king::King::validate_and_execute_move(&chess_move, &board),
            PieceType::Rook => rook::Rook::validate_and_execute_move(&chess_move, &board),
            PieceType::Bishop => bishop::Bishop::validate_and_execute_move(&chess_move, &board),
            PieceType::Pawn => pawn::Pawn::validate_and_execute_move(&chess_move, &board),
            PieceType::Knight => knight::Knight::validate_and_execute_move(&chess_move, &board),
        };

        if is_valid {
            let player = piece.1;
            Self::is_check(&player, &new_board);
        }

        (is_valid, new_board)
    }

    fn is_check(player: &Player, board: &Board) -> bool {
        for (piece, pos) in BoardIterator::new(board) {
            if piece != Piece(PieceType::King, *player) {
                continue;
            }

            // Column

            for y in (pos.rank() + 1..9).chain(1..pos.rank()) {
                match board.get_square(&BoardPosition::new(pos.file(), y)) {
                    Some(square) => if
                        square.1 == *player ||
                        (square.0 != PieceType::Rook && square.0 != PieceType::Queen)
                    {
                        break;
                    } else if square.0 == PieceType::Rook {
                        return true;
                    }
                    None => todo!(),
                }
            }

            // Row

            for x in (pos.file() + 1..9).chain(1..pos.file()) {
                match board.get_square(&BoardPosition::new(x, pos.rank())) {
                    Some(square) => if
                        square.1 == *player ||
                        (square.0 != PieceType::Rook && square.0 != PieceType::Queen)
                    {
                        break;
                    } else if square.0 == PieceType::Rook {
                        return true;
                    }
                    None => todo!(),
                }
            }

            // Diagonal

            for (x, y) in (1u8..9u8)
                .map(|x| (x, pos.rank() + x.abs_diff(pos.file())))
                .chain((1u8..9u8).map(|x| (x, pos.rank() - x.abs_diff(pos.file())))) {
                let piece = match board.get_square(&BoardPosition::new(x, y)) {
                    Some(piece) => piece,
                    None => {
                        continue;
                    }
                };

                if piece == &Piece(PieceType::King, *player) {
                    continue;
                }

                if &piece.1 == player {
                    break;
                }

                if
                    (piece.0 == PieceType::Bishop ||
                        piece.0 == PieceType::Queen ||
                        piece.0 == PieceType::Pawn) &&
                    Self::raw_validate_move(
                        &(Move { from: BoardPosition::from((x, y)), to: pos.clone() }),
                        board,
                        piece.0
                    )
                {
                    return true;
                }
            }

            // Knight

            for knight_pos in [
                (1, 3),
                (-1, 3),
                (1, -3),
                (-1, -3),
                (3, 1),
                (3, -1),
                (-3, 1),
                (-3, -1),
            ]
                .iter()
                .map(|(x, y)| ((pos.file() as i8) + x, (pos.rank() as i8) + y))
                .filter(|(x, y)| (1..9).contains(x) && (1..9).contains(y))
                .map(|(x, y)| BoardPosition::new(x as u8, y as u8)) {
                if
                    board.get_square(&knight_pos) ==
                    &Some(Piece(PieceType::Knight, player.opposite()))
                {
                    return true;
                }
            }
        }

        false
    }

    fn raw_validate_move(chess_move: &Move, board: &Board, piece: PieceType) -> bool {
        let (from, to) = (chess_move.from.tuple(), chess_move.to.tuple());

        match piece {
            PieceType::Queen => queen::Queen::validate_move(from, to, &board),
            PieceType::King => king::King::validate_move(from, to, &board),
            PieceType::Rook => rook::Rook::validate_move(from, to, &board),
            PieceType::Bishop => bishop::Bishop::validate_move(from, to, &board),
            PieceType::Pawn => pawn::Pawn::validate_move(from, to, &board),
            PieceType::Knight => knight::Knight::validate_move(from, to, &board),
        }
    }
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece(pub PieceType, pub Player);

#[cfg(test)]
mod tests {
    #[test]
    fn check() {}
}