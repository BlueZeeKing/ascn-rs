use crate::pieces::Piece;

#[derive(Clone)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
}

impl Board {
    // pub fn apply_move(&mut self, game_move: Move) -> Result<(), MoveError> {
    //     let (x, y) = game_move.from.get_index();
    //     self.board[y as usize][x as usize] = None;

    //     let (x, y) = game_move.to.get_index();
    //     self.board[y as usize][x as usize] = None;
    // }

    pub fn set_square(&mut self, pos: BoardPosition, value: Option<Piece>) {
        let (x, y) = pos.get_index();
        self.board[y as usize][x as usize] = value;
    }

    pub fn get_square(&self, pos: BoardPosition) -> &Option<Piece> {
        let (x, y) = pos.get_index();
        &self.board[y as usize][x as usize]
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: BoardPosition,
    pub to: BoardPosition,
}

#[derive(Copy, Clone)]
pub struct BoardPosition(u8, u8);

impl BoardPosition {
    pub fn get_index(&self) -> (u8, u8) {
        (((self.0 as i8) - 8).abs() as u8, self.1 - 1)
    }

    pub fn tuple(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    pub fn from(pos: (u8, u8)) -> Self {
        Self(pos.0, pos.1)
    }

    pub fn new(x: u8, y: u8) -> Self {
        Self(x, y)
    }
}

pub enum MoveError {}