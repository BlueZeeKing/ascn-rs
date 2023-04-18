use shakmaty::{CastlingSide, Chess, File, Move, Piece, Position, Rank, Role, Square};

use super::Filter;

pub struct Straight {}

impl Filter for Straight {
    fn get_raw_id() -> u8 {
        3
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>> {
        let mut square_data: Vec<Option<Square>> = vec![None; 4];

        for rank in i32::from(to.rank()) + 1..8 {
            // up
            if vertical_checks(0, &mut square_data, to, position, rank) {
                break;
            }
        }

        for rank in (0..i32::from(to.rank())).rev() {
            // down
            if vertical_checks(2, &mut square_data, to, position, rank) {
                break;
            }
        }

        for file in i32::from(to.file()) + 1..8 {
            // right
            if horizontal_checks(1, &mut square_data, to, position, file) {
                break;
            }
        }

        for file in (0..i32::from(to.file())).rev() {
            // left
            if horizontal_checks(3, &mut square_data, to, position, file) {
                break;
            }
        }

        square_data
    }
}

fn vertical_checks(
    index: usize,
    square_data: &mut [Option<Square>],
    to: &Square,
    position: &Chess,
    rank: i32,
) -> bool {
    let rank = Rank::try_from(rank).unwrap();
    let square = Square::from_coords(to.file(), rank);

    let possible_piece = position.board().piece_at(square);

    let piece: Piece;

    if let Some(temp_piece) = possible_piece {
        piece = temp_piece;
    } else {
        return false;
    }

    if (piece.role == Role::Rook
        || piece.role == Role::Queen
        || (piece.role == Role::King && rank.distance(to.rank()) == 1))
        && position.is_legal(
            &(Move::Normal {
                role: piece.role,
                from: square,
                capture: position.board().piece_at(*to).map(|piece| piece.role),
                to: *to,
                promotion: None,
            }),
        )
    {
        square_data[index] = Some(square);
        return true;
    }

    if piece.role == Role::Pawn {
        let should_move_forward = piece.color.is_white();

        if (should_move_forward && square.rank() > to.rank())
            || (!should_move_forward && square.rank() < to.rank())
        {
            return true;
        }

        let starting_rank = if piece.color.is_white() {
            Rank::Second
        } else {
            Rank::Seventh
        };

        if ((rank.distance(to.rank()) == 2 && rank == starting_rank)
            || rank.distance(to.rank()) == 1)
            && position.is_legal(
                &(Move::Normal {
                    role: Role::Pawn,
                    from: square,
                    capture: None,
                    to: *to,
                    promotion: if to.rank() == Rank::First || to.rank() == Rank::Eighth {
                        Some(Role::Queen)
                    } else {
                        None
                    },
                }),
            )
        {
            square_data[index] = Some(square);
        }
    }

    true
}

fn horizontal_checks(
    index: usize,
    square_data: &mut [Option<Square>],
    to: &Square,
    position: &Chess,
    file: i32,
) -> bool {
    let file = File::try_from(file).unwrap_or_else(|_| panic!("Could not parse number: {file}"));
    let square = Square::from_coords(file, to.rank());

    let possible_piece = position.board().piece_at(square);

    let piece: Piece;

    if let Some(temp_piece) = possible_piece {
        piece = temp_piece;
    } else {
        return false;
    }

    if (piece.role == Role::Rook
        || piece.role == Role::Queen
        || (piece.role == Role::King && file.distance(to.file()) == 1))
        && position.is_legal(
            &(Move::Normal {
                role: piece.role,
                from: square,
                capture: position.board().piece_at(*to).map(|piece| piece.role),
                to: *to,
                promotion: None,
            }),
        )
    {
        square_data[index] = Some(square);
        return true;
    }

    if piece.role == Role::King && file.distance(to.file()) == 2 {
        let castles = position.castles();

        let side = if to.file() == File::G {
            CastlingSide::KingSide
        } else {
            CastlingSide::QueenSide
        };

        if castles.has(piece.color, side)
            && position.is_legal(
                &(Move::Castle {
                    king: square,
                    rook: Square::from_coords(
                        if side.is_king_side() {
                            File::H
                        } else {
                            File::A
                        },
                        square.rank(),
                    ),
                }),
            )
        {
            square_data[index] = Some(square);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use shakmaty::{Chess, Color, Move, Piece, Position, Role::*, Setup, Square::*};

    use crate::filters::Filter;

    use super::Straight;

    #[test]
    fn simple_square_data_test() {
        let chess = Chess::default();

        assert_eq!(
            Straight::get_square_data(&A4, &chess),
            vec![None, None, Some(A2), None]
        );
    }

    #[test]
    fn castling_square_data_test() {
        let chess = Chess::default()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: D2,
                    capture: None,
                    to: D3,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: H7,
                    capture: None,
                    to: H6,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Bishop,
                    from: C1,
                    capture: None,
                    to: D2,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: G7,
                    capture: None,
                    to: G6,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Knight,
                    from: B1,
                    capture: None,
                    to: C3,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: F7,
                    capture: None,
                    to: F6,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: E2,
                    capture: None,
                    to: E3,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: E7,
                    capture: None,
                    to: E6,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Queen,
                    from: D1,
                    capture: None,
                    to: E2,
                    promotion: None,
                }),
            )
            .unwrap()
            .play(
                &(Move::Normal {
                    role: Pawn,
                    from: D7,
                    capture: None,
                    to: D6,
                    promotion: None,
                }),
            )
            .unwrap();

        assert_eq!(
            Straight::get_square_data(&C1, &chess),
            vec![None, Some(E1), None, Some(A1)]
        );
    }

    #[test]
    fn complex_square_data_test() {
        let mut setup = Setup::empty();
        setup.board.set_piece_at(
            D4,
            Piece {
                color: Color::Black,
                role: Pawn,
            },
        );
        setup.board.set_piece_at(
            H4,
            Piece {
                color: Color::White,
                role: Rook,
            },
        );
        setup.board.set_piece_at(
            D1,
            Piece {
                color: Color::White,
                role: Queen,
            },
        );
        setup.board.set_piece_at(
            D5,
            Piece {
                color: Color::White,
                role: King,
            },
        );
        setup.board.set_piece_at(
            A8,
            Piece {
                color: Color::Black,
                role: King,
            },
        );
        setup.board.set_piece_at(
            A2,
            Piece {
                color: Color::White,
                role: Pawn,
            },
        );

        let chess: Chess = setup.position(shakmaty::CastlingMode::Standard).unwrap();

        assert_eq!(
            Straight::get_square_data(&D4, &chess),
            vec![Some(D5), Some(H4), Some(D1), None]
        );
        assert_eq!(
            Straight::get_square_data(&A4, &chess),
            vec![None, None, Some(A2), None]
        );
    }
}
