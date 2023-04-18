use std::fs::File;

use ascn_rs::{ writer::Writer, reader::Reader };
use indicatif::ProgressBar;
use pgn_reader::{ Visitor, BufferedReader };
use shakmaty::{ Chess, Position };

const NUM_GAMES: u64 = 121332;

struct TestVisitor {
    chess: Chess,
    writer: Writer,
    progress_bar: ProgressBar,
}

impl TestVisitor {
    fn new() -> Self {
        Self {
            chess: Chess::default(),
            writer: Writer::new(),
            progress_bar: ProgressBar::new(NUM_GAMES),
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
        let (_chess_move, game) = Reader::new(&self.writer.clone().get_data()).last().unwrap();
        assert_eq!(game, self.chess);
        self.progress_bar.inc(1);
    }
}

#[test]
#[ignore]
fn test() {
    let mut visitor = TestVisitor::new();
    let mut reader = BufferedReader::new(File::open("tests/lichess_test_collection.pgn").unwrap());

    reader.read_all(&mut visitor).unwrap();
}