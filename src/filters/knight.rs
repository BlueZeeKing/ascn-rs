use shakmaty::{ Square, Chess, Position, Rank, Role, Piece, Board, Color, File, CastlingSide };

use super::Filter;

pub struct Knight {}

impl Filter for Knight {
    fn get_id() -> u8 {
        1
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>> {
        let mut square_data: Vec<Option<Square>> = vec![None; 8];

        for (index, square) in [
            (3, 1),
            (3, -1),
            (-3, 1),
            (-3, -1),
            (1, 3),
            (1, -3),
            (-1, 3),
            (-1, -3),
        ]
            .iter()
            .map(|(file, rank)|
                Square::from_coords(
                    File::try_from(i32::from(to.file()) + file).unwrap(),
                    Rank::try_from(i32::from(to.rank()) + rank).unwrap()
                )
            )
            .enumerate() {
            if
                position
                    .board()
                    .piece_at(square)
                    .unwrap_or(Piece { color: Color::White, role: Role::Pawn }).role == Role::Knight
            {
                square_data[index] = Some(square);
            }
        }

        square_data
    }

    fn get_overflow(to: &Square, from: &Square, position: &Chess) -> Option<(u8, u8)> {
        // the number of the overflow bits
        let square_data = Self::get_square_data(to, position);

        if
            square_data
                .iter()
                .filter(|square| square.is_some())
                .count() == 1
        {
            None
        } else {
            Some((
                square_data
                    .iter()
                    .position(|square| (
                        if let Some(square) = square {
                            square == from
                        } else {
                            false
                        }
                    ))
                    .expect("Could not find original square") as u8,
                3,
            ))
        }
    }

    fn get_overflow_mask() -> u8 {
        0b00000111
    }
}