use shakmaty::{Chess, File, Move, Position, Rank, Role, Square};

use crate::{
    bitbuffer::BitBuffer,
    filters::{diagonal::Diagonal, knight::Knight, straight::Straight, Filter},
    PROMOTION_KEY,
};

pub struct Reader {
    data: Vec<u8>,
    chess: Chess,
    bit_buffer: BitBuffer,
}

impl Reader {
    /// Creates a new reader based on the buffer provided
    /// It also creates a new default chess board
    pub fn new(data: &[u8]) -> Self {
        Reader {
            data: data.to_vec(),
            chess: Chess::default(),
            bit_buffer: BitBuffer::from_bytes(data),
        }
    }
}

impl Iterator for Reader {
    /// Returns the next move processed and the current state of the board after the move has been applied
    fn next(&mut self) -> Option<Self::Item> {
        let to = Square::try_from(self.data[0] & 0b00111111).expect("Your computer was hit with a radioactive particle and made a 6 bit number greater that 63");
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

        if let Some(i) = square_data.iter().position(|square| square.is_some()) {
            index = i;
        } else {
            index = self.bit_buffer.read(overflow_length) as usize;
        }

        let from = square_data[index].expect("Could not find valid move from overflow index");

        let from_piece =
            self.chess.board().piece_at(from).expect(
                "Could not find piece at previously validated square (radioactive particle?)",
            );

        let chess_move = if id == 2
            && from_piece.role == Role::Pawn
            && self.chess.board().piece_at(to).is_none()
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
                    from.rank(),
                ),
            }
        } else {
            Move::Normal {
                role: from_piece.role,
                from,
                capture: self.chess.board().piece_at(to).map(|piece| piece.role),
                to,
                promotion: if from_piece.role == Role::Pawn
                    && (to.rank() == Rank::First || to.rank() == Rank::Eighth)
                {
                    Some(PROMOTION_KEY[self.bit_buffer.read(2) as usize])
                } else {
                    None
                },
            }
        };

        self.chess =
            self.chess.clone().play(&chess_move).expect(
                "Invalid move while reading (Should not be possible even with modified input)",
            );

        Some((chess_move, self.chess.clone()))
    }

    type Item = (Move, Chess);
}
