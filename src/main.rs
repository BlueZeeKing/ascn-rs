use chess::board::{ Board, position::BoardPosition };

fn main() {
    let board = Board::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    ).unwrap();
    println!("{:?}", board.get_square(&BoardPosition::new(1, 1)));
    println!("{}", board)
}