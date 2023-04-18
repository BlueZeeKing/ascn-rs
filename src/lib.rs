use shakmaty::Role;

mod bitbuffer;
mod filters;
pub mod reader;
pub mod writer;

const PROMOTION_KEY: [Role; 4] = [Role::Queen, Role::Bishop, Role::Rook, Role::Knight];
