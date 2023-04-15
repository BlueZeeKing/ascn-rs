use shakmaty::{ Square, Chess };

pub mod straight;
pub mod diagonal;
pub mod knight;

pub trait Filter {
    fn get_id() -> u8;

    fn get_raw_id() -> u8 {
        Self::get_id() << 6
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
                2,
            ))
        }
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>>;

    fn get_overflow_mask() -> u8 {
        0b00000011
    }
}