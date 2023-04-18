use shakmaty::{Chess, Color, File, Move, Piece, Position, Rank, Role, Square};

use super::Filter;

pub struct Knight {}

impl Filter for Knight {
    fn get_raw_id() -> u8 {
        1
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>> {
        let mut square_data: Vec<Option<Square>> = vec![None; 8];

        for (index, square) in [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
        ]
        .iter()
        .filter(|(file, rank)| {
            (0..8).contains(&(i32::from(to.file()) + file))
                && (0..8).contains(&(i32::from(to.rank()) + rank))
        })
        .map(|(file, rank)| {
            Square::from_coords(
                File::try_from(i32::from(to.file()) + file).unwrap(),
                Rank::try_from(i32::from(to.rank()) + rank).unwrap(),
            )
        })
        .enumerate()
        {
            if position
                .board()
                .piece_at(square)
                .unwrap_or(Piece {
                    color: Color::White,
                    role: Role::Pawn,
                })
                .role
                == Role::Knight
                && position.is_legal(
                    &(Move::Normal {
                        role: Role::Knight,
                        from: square,
                        capture: position.board().piece_at(*to).map(|piece| piece.role),
                        to: *to,
                        promotion: None,
                    }),
                )
            {
                square_data[index] = Some(square);
            }
        }

        square_data
    }

    fn get_overflow(to: &Square, from: &Square, position: &Chess) -> Option<(u8, u8)> {
        // the number of the overflow bits
        let square_data = Self::get_square_data(to, position);

        if square_data.iter().filter(|square| square.is_some()).count() == 1 {
            None
        } else {
            Some((
                square_data
                    .iter()
                    .position(|square| {
                        if let Some(square) = square {
                            square == from
                        } else {
                            false
                        }
                    })
                    .expect("Could not find original square") as u8,
                3,
            ))
        }
    }

    fn get_overflow_mask() -> u8 {
        0b00000111
    }
}

#[cfg(test)]
mod tests {
    use shakmaty::{Chess, Color, Piece, Role::*, Setup, Square::*};

    use crate::filters::{knight::Knight, Filter};

    #[test]
    fn square_data_test() {
        let mut setup = Setup::empty();
        setup.board.set_piece_at(
            A1,
            Piece {
                color: Color::White,
                role: King,
            },
        );
        setup.board.set_piece_at(
            H8,
            Piece {
                color: Color::Black,
                role: King,
            },
        );

        setup.board.set_piece_at(
            B5,
            Piece {
                color: Color::White,
                role: Knight,
            },
        );
        setup.board.set_piece_at(
            C2,
            Piece {
                color: Color::White,
                role: Knight,
            },
        );
        setup.board.set_piece_at(
            E6,
            Piece {
                color: Color::White,
                role: Knight,
            },
        );
        setup.board.set_piece_at(
            F5,
            Piece {
                color: Color::White,
                role: Knight,
            },
        );
        setup.board.set_piece_at(
            F3,
            Piece {
                color: Color::White,
                role: Knight,
            },
        );

        let chess: Chess = setup.position(shakmaty::CastlingMode::Standard).unwrap();

        assert_eq!(
            Knight::get_square_data(&D4, &chess),
            vec![
                Some(F5),
                Some(F3),
                Some(B5),
                None,
                Some(E6),
                None,
                None,
                Some(C2)
            ]
        );
    }
}
