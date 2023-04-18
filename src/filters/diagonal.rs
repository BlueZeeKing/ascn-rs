use shakmaty::{ Square, Chess, Position, Rank, Role, Piece, File, Move };

use super::Filter;

pub struct Diagonal {}

impl Filter for Diagonal {
    fn get_id() -> u8 {
        2
    }

    fn get_square_data(to: &Square, position: &Chess) -> Vec<Option<Square>> {
        let mut square_data: Vec<Option<Square>> = vec![None; 4];

        for square in (i32::from(to.rank()) + 1..8)
            .zip(i32::from(to.file()) + 1..8)
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // up and right
            if diagonal_checks(0, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (0..i32::from(to.rank()))
            .rev()
            .zip(i32::from(to.file()) + 1..8)
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // down and right
            if diagonal_checks(1, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (0..i32::from(to.rank()))
            .rev()
            .zip((0..i32::from(to.file())).rev())
            .map(|(rank, file)|
                Square::from_coords(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            ) {
            // right
            if diagonal_checks(2, &mut square_data, to, position, square) {
                break;
            }
        }

        for square in (i32::from(to.rank()) + 1..8)
            .zip((0..i32::from(to.file())).rev())
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

#[cfg(test)]
mod tests {
    use shakmaty::{ Chess, Square::*, Setup, Role::*, Piece, Color, Position, Move };

    use crate::filters::{ diagonal::Diagonal, Filter };

    #[test]
    fn square_data_test() {
        let mut setup = Setup::empty();
        setup.board.set_piece_at(D3, Piece { color: Color::White, role: Pawn });
        setup.board.set_piece_at(H1, Piece { color: Color::White, role: Bishop });
        setup.board.set_piece_at(G6, Piece { color: Color::White, role: Queen });
        setup.board.set_piece_at(D5, Piece { color: Color::White, role: King });

        setup.board.set_piece_at(A8, Piece { color: Color::Black, role: King });

        setup.board.set_piece_at(A2, Piece { color: Color::White, role: Pawn });
        setup.board.set_piece_at(B3, Piece { color: Color::Black, role: Pawn });

        let chess: Chess = setup.position(shakmaty::CastlingMode::Standard).unwrap();

        assert_eq!(Diagonal::get_square_data(&B3, &chess), vec![None, None, Some(A2), None]);
        assert_eq!(
            Diagonal::get_square_data(&E4, &chess),
            vec![Some(G6), Some(H1), None, Some(D5)]
        );
    }

    #[test]
    fn ep_square_data_test() {
        let chess = Chess::default()
            .play(&(Move::Normal { role: Pawn, from: A2, capture: None, to: A4, promotion: None }))
            .unwrap()
            .play(&(Move::Normal { role: Pawn, from: H7, capture: None, to: H6, promotion: None }))
            .unwrap()
            .play(&(Move::Normal { role: Pawn, from: A4, capture: None, to: A5, promotion: None }))
            .unwrap()
            .play(&(Move::Normal { role: Pawn, from: B7, capture: None, to: B5, promotion: None }))
            .unwrap();

        assert_eq!(Diagonal::get_square_data(&B6, &chess), vec![None, None, Some(A5), None]);
    }
}