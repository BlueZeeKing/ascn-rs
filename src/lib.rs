use shakmaty::Role;

mod bitbuffer;
pub mod filters;
pub mod reader;
pub mod writer;

pub const PROMOTION_KEY: [Role; 4] = [Role::Queen, Role::Bishop, Role::Rook, Role::Knight];
