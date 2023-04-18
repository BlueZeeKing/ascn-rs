use std::fs::File;

use chess::{ writer::Writer, reader::Reader };
use pgn_reader::{ Visitor, BufferedReader };
use shakmaty::{ Chess, Square, Role, Position, fen::Fen };

struct TestVisitor {
    chess: Chess,
    writer: Writer,
}

impl TestVisitor {
    fn new() -> Self {
        Self { chess: Chess::default(), writer: Writer::new() }
    }
}

impl Visitor for TestVisitor {
    fn san(&mut self, _san_plus: pgn_reader::SanPlus) {
        let chess_move = _san_plus.san.to_move(&self.chess).unwrap();

        self.writer.add_move(&chess_move, &self.chess);
        self.chess = self.chess.clone().play(&chess_move).unwrap();
    }

    type Result = Vec<u8>;

    fn end_game(&mut self) -> Self::Result {
        self.writer.clone().get_data()
    }
}

fn main() {
    let mut visitor = TestVisitor::new();
    let mut reader = BufferedReader::new(File::open("test.pgn").unwrap());

    let output = reader.read_game(&mut visitor).unwrap().unwrap();

    let reader = Reader::new(&output);

    let mut chess = Chess::default();

    for (_chess_move, new_position) in reader {
        chess = new_position;
    }

    assert_eq!(
        chess,
        "4K3/4N1k1/5p1n/3Q4/q7/8/8/8 w - - 2 76"
            .parse::<Fen>()
            .unwrap()
            .0.position(shakmaty::CastlingMode::Standard)
            .unwrap()
    )
}