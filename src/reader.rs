use shakmaty::{ Move, Square, Chess, Position, Role, File, Rank };

use crate::{
    filters::{ straight::Straight, diagonal::Diagonal, knight::Knight, Filter },
    bitbuffer::BitBuffer,
    PROMOTION_KEY,
};

pub struct Reader {
    data: Vec<u8>,
    chess: Chess,
    bit_buffer: BitBuffer,
}

impl Reader {
    pub fn new(data: &[u8]) -> Self {
        Reader {
            data: data.to_vec(),
            chess: Chess::default(),
            bit_buffer: BitBuffer::from_bytes(data),
        }
    }
}

impl Iterator for Reader {
    fn next(&mut self) -> Option<Self::Item> {
        let to = Square::try_from(self.data[0] & 0b00111111).unwrap();
        let id = self.data[0] >> 6;

        let (square_data, overflow_length) = match id {
            3 => (Straight::get_square_data(&to, &self.chess), 2),
            2 => (Diagonal::get_square_data(&to, &self.chess), 2),
            1 => (Knight::get_square_data(&to, &self.chess), 3),
            0 => {
                return None;
            }
            _ => panic!("Unknown filter"),
        };

        self.data.remove(0);

        let index;

        if
            square_data
                .iter()
                .filter(|square| square.is_some())
                .count() == 1
        {
            index = square_data
                .iter()
                .position(|square| square.is_some())
                .unwrap();
        } else {
            index = self.bit_buffer.read(overflow_length) as usize;
        }

        let from = square_data[index].unwrap();

        let from_piece = self.chess.board().piece_at(from).unwrap();

        let chess_move = if
            id == 2 &&
            from_piece.role == Role::Pawn &&
            self.chess.board().piece_at(to).is_none()
        {
            Move::EnPassant { from, to }
        } else if id == 3 && from_piece.role == Role::King && from.file().distance(to.file()) == 2 {
            Move::Castle {
                king: from,
                rook: Square::from_coords(
                    if to.file() == File::C {
                        File::A
                    } else {
                        File::H
                    },
                    from.rank()
                ),
            }
        } else {
            Move::Normal {
                role: from_piece.role,
                from,
                capture: self.chess
                    .board()
                    .piece_at(to).map(|piece| piece.role),
                to,
                promotion: if
                    from_piece.role == Role::Pawn &&
                    (to.rank() == Rank::First || to.rank() == Rank::Eighth)
                {
                    Some(PROMOTION_KEY[self.bit_buffer.read(2) as usize])
                } else {
                    None
                },
            }
        };

        self.chess = self.chess.clone().play(&chess_move).unwrap();

        Some((chess_move, self.chess.clone()))
    }

    type Item = (Move, Chess);
}