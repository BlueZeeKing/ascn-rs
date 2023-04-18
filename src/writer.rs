use shakmaty::{ Move, Chess, Square, File };

use crate::{
    filters::{ straight::Straight, Filter, diagonal::Diagonal, knight::Knight },
    bitbuffer::BitBuffer,
};

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
            .chain(Self::get_overflow_data(self.overflow).iter()).copied()
            .collect()
    }

    fn get_overflow_data(overflow: Vec<(u8, u8)>) -> Vec<u8> {
        let mut bit_buffer = BitBuffer::new();

        for (data, length) in overflow {
            bit_buffer.add(data, length);
        }

        bit_buffer.to_bytes()
    }

    pub fn add_move(&mut self, chess_move: &Move, position: &Chess) {
        let to_square: Square;
        let id: u8;
        let overflow: Option<(u8, u8)>; // data, num bits

        match chess_move {
            Move::Normal { role: _, from, capture: _, to, promotion: _ } => {
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
            Move::Put { role: _, to: _ } => todo!(),
        }

        self.core.push(u8::from(to_square) | id);

        if let Some(data) = overflow {
            self.overflow.push(data);
        }
    }
}