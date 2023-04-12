use crate::pieces::Piece;

use super::{ Board, position::BoardPosition };

pub struct BoardIterator<'a> {
    board: &'a Board,

    x: usize,
    y: usize,
}

impl<'a> BoardIterator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board, x: 0, y: 0 }
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Piece, BoardPosition);

    fn next(&mut self) -> Option<Self::Item> {
        while self.board.board[self.y][self.x].is_none() {
            self.x += 1;

            if self.x == 8 && self.y == 7 {
                return None;
            } else if self.x == 8 {
                self.x = 0;
                self.y += 1;
            }
        }

        Some((
            self.board.board[self.y][self.x].expect("Piece was found but is not gone"),
            BoardPosition::from_index((self.x as u8, self.y as u8)),
        ))
    }
}