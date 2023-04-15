use shakmaty::{ Move, Chess, Square, File };

use crate::filters::{ straight::Straight, Filter, diagonal::Diagonal, knight::Knight };

pub struct Writer {
    core: Vec<u8>,
    overflow: Vec<(u8, u8)>, // data, # of bits
}

impl Writer {
    pub fn new() -> Self {
        Self { core: Vec::new(), overflow: Vec::new() }
    }

    pub fn get_data(self) -> Vec<u8> {
        self.core
            .iter()
            .chain(Self::get_overflow_data(self.overflow).iter())
            .map(|byte| *byte)
            .collect()
    }

    fn get_overflow_data(overflow: Vec<(u8, u8)>) -> Vec<u8> {
        // FIXME: This doesn't work
        let mut data = vec![0x00];
        let mut byte_position = 0;

        for (data_part, length) in overflow.iter().rev() {
            let mut data_byte = data[0];
            data_byte |= data_part << byte_position;
            data[0] = data_byte;

            dbg!(&data);

            byte_position += length;

            if byte_position == 8 {
                byte_position = 0;
                data.insert(0, 0x00);
            } else if byte_position > 8 {
                data.insert(0, 0x00);

                let data_read = length - (byte_position - 8);

                let mut data_byte = data[0];
                data_byte |= data_part >> data_read;
                data[0] = data_byte;

                byte_position -= 8;
            }
        }

        data
    }

    pub fn add_move(&mut self, chess_move: &Move, position: &Chess) {
        let to_square: Square;
        let id: u8;
        let overflow: Option<(u8, u8)>; // data, num bits

        match chess_move {
            Move::Normal { role, from, capture, to, promotion } => {
                to_square = *to;

                if from.rank() == to.rank() || from.file() == to.file() {
                    id = Straight::get_id();
                    overflow = Straight::get_overflow(to, from, position);
                } else if from.rank().distance(to.rank()) == from.file().distance(to.file()) {
                    id = Diagonal::get_id();
                    overflow = Diagonal::get_overflow(to, from, position);
                } else {
                    id = Knight::get_id();
                    overflow = Knight::get_overflow(to, from, position);
                }
            }
            Move::EnPassant { from, to } => {
                to_square = *to;
                id = Diagonal::get_id();
                overflow = Diagonal::get_overflow(to, from, position);
            }
            Move::Castle { king, rook } => {
                to_square = Square::from_coords(
                    File::try_from(
                        i8::from(king.file()) + (if rook.file() < king.file() { -2 } else { 2 })
                    ).expect("Could not get final king square for castling move"),
                    king.rank()
                );

                id = Straight::get_id();
                overflow = Straight::get_overflow(&to_square, king, position);
            }
            Move::Put { role, to } => todo!(),
        }

        self.core.push(u8::from(to_square) | id);

        if let Some(data) = overflow {
            self.overflow.push(data);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::writer::Writer;

    #[test]
    fn overflow_join() {
        assert_eq!(Writer::get_overflow_data(vec![(0b111, 3)]), vec![0b111]);
        assert_eq!(
            Writer::get_overflow_data(vec![(0b1111, 4), (0b0000, 4)]),
            vec![0b0, 0b11110000]
        );
        // assert_eq!(
        //     Writer::get_overflow_data(vec![(0b111, 3), (0b000, 3), (0b10101, 5)]),
        //     vec![0b11100010, 0b101]
        // );
    }
}