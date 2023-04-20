use std::fs::read_to_string;

use ascn_rs::{reader::Reader, writer::Writer};
use chess::Board;
use indicatif::ProgressBar;
use pgn_rs::{san::SAN, PGNReader, Visitor};

const NUM_GAMES: u64 = 121332;

struct TestVisitor {
    chess: Board,
    writer: Writer,
    progress_bar: ProgressBar,
}

impl TestVisitor {
    fn new() -> Self {
        Self {
            chess: Board::default(),
            writer: Writer::new(),
            progress_bar: ProgressBar::new(NUM_GAMES),
        }
    }
}

impl Visitor for TestVisitor {
    fn start_game(&mut self) {
        self.chess = Board::default();
        self.writer = Writer::new();
    }

    fn san(&mut self, san: SAN) {
        let chess_move = san.to_move(&self.chess);

        self.writer.add_move(&chess_move, &self.chess);
        self.chess = self.chess.make_move_new(chess_move);
    }

    fn end_game(&mut self) {
        let (_, board) = Reader::new(&self.writer.clone().get_data()).last().unwrap();
        assert_eq!(self.chess, board);
        self.progress_bar.inc(1);
    }

    fn header(&mut self, _header: pgn_rs::Header) {}
}

#[test]
#[ignore]
fn test() {
    let mut visitor = TestVisitor::new();

    let data = read_to_string("tests/lichess_test_collection.pgn").unwrap();

    let reader = PGNReader::new(&data);

    reader.read(&mut visitor);
}
