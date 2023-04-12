#[derive(Clone, Debug)]
pub struct BoardPosition(u8, u8);

impl BoardPosition {
    pub fn get_index(&self) -> (u8, u8) {
        (self.0 - 1, self.1 - 1)
    }

    pub fn from_index(pos: (u8, u8)) -> Self {
        BoardPosition(pos.0 + 1, pos.1 + 1)
    }

    pub fn tuple(&self) -> (u8, u8) {
        (self.0, self.1)
    }

    pub fn from(pos: (u8, u8)) -> Self {
        Self(pos.0, pos.1)
    }

    pub fn new(file: u8, rank: u8) -> Self {
        Self(file, rank)
    }

    pub fn file(&self) -> u8 {
        self.0
    }

    pub fn rank(&self) -> u8 {
        self.1
    }
}

#[derive(Clone)]
pub struct Move {
    pub from: BoardPosition,
    pub to: BoardPosition,
}