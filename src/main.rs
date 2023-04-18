use std::{ fs::File, io::Write };

use chess::{ writer::Writer, reader::Reader };

use pgn_reader::{ Visitor, BufferedReader };
use shakmaty::{ Chess, Position };

struct TestVisitor {
    chess: Chess,
    writer: Writer,
}

impl TestVisitor {
    fn new() -> Self {
        Self {
            chess: Chess::default(),
            writer: Writer::new(),
        }
    }
}

impl Visitor for TestVisitor {
    fn begin_game(&mut self) {
        self.chess = Chess::default();
        self.writer = Writer::new();
    }

    fn san(&mut self, _san_plus: pgn_reader::SanPlus) {
        let chess_move = _san_plus.san.to_move(&self.chess).unwrap();

        self.writer.add_move(&chess_move, &self.chess);
        self.chess = self.chess.clone().play(&chess_move).unwrap();
    }

    type Result = ();

    fn end_game(&mut self) -> Self::Result {
        let mut output = File::create("test.chess").unwrap();
        output.write(&self.writer.clone().get_data());
        output.flush();

        let (_chess_move, game) = Reader::new(&self.writer.clone().get_data()).last().unwrap();
        assert_eq!(game, self.chess);
    }
}

fn main() {
    let mut visitor = TestVisitor::new();
    let mut reader = BufferedReader::new(File::open("test.pgn").unwrap());

    reader.read_all(&mut visitor).unwrap();
}