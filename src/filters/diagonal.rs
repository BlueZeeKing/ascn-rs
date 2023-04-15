use shakmaty::{
    Square,
    Chess,
    Position,
    Rank,
    Role,
    Piece,
    Board,
    Color,
    File,
    CastlingSide,
    Move,
};

use super::Filter;

pub struct Diagonal {}

impl Filter for Diagonal {
    fn get_id() -> u8 {
        2
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>> {
        let mut square_data: Vec<Option<Square>> = vec![None; 4];

        for square in (i32::from(to.rank()) + 1..9)
            .zip(i32::from(to.file()) + 1..9)
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // up and right
            if diagonal_checks(0, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (1..i32::from(to.rank()))
            .rev()
            .zip(i32::from(to.file()) + 1..9)
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // down and right
            if diagonal_checks(1, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (1..i32::from(to.rank()))
            .rev()
            .zip((1..i32::from(to.file())).rev())
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // right
            if diagonal_checks(2, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (i32::from(to.rank()) + 1..9)
            .zip((1..i32::from(to.file())).rev())
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // left
            if diagonal_checks(3, &mut square_data, to, position, square) {
                break;
            }
        }

        square_data
    }
}

fn diagonal_checks(
    index: usize,
    square_data: &mut [Option<Square>],
    to: &Square,
    position: &Chess,
    square: Square
) -> bool {
    let possible_piece = position.board().piece_at(square);

    let piece: Piece;

    if let Some(temp_piece) = possible_piece {
        piece = temp_piece;
    } else {
        return false;
    }

    if
        piece.role == Role::Bishop ||
        piece.role == Role::Queen ||
        (piece.role == Role::King && square.rank().distance(to.rank()) == 1)
    {
        square_data[index] = Some(square);
        return true;
    }

    if piece.role == Role::Pawn && square.rank().distance(to.rank()) == 1 {
        let should_move_forward = piece.color.is_white();

        if
            (should_move_forward && square.rank() > to.rank()) ||
            (!should_move_forward && square.rank() < to.rank())
        {
            return true;
        }

        if
            position.board().piece_at(*to).is_some() ||
            position.en_passant_moves().contains(&(Move::EnPassant { from: square, to: *to }))
        {
            square_data[index] = Some(square);
        }
    }

    true
}