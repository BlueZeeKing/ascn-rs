use shakmaty::Role;

pub mod writer;
pub mod filters;
mod bitbuffer;
pub mod reader;

pub const PROMOTION_KEY: [Role; 4] = [Role::Queen, Role::Bishop, Role::Rook, Role::Knight];