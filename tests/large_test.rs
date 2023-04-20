use std::fs::read_to_string;

use ascn_rs::{outcome::Outcome, reader::Reader, writer::Writer};
use chess::Board;
use indicatif::ProgressBar;
use pgn_rs::{san::SAN, PGNReader, Visitor};

const NUM_GAMES: u64 = 121332;

struct TestVisitor {
    chess: Board,
    writer: Writer,
    progress_bar: ProgressBar,
    outcome: Option<Outcome>,
}

impl TestVisitor {
    fn new() -> Self {
        Self {
            chess: Board::default(),
            writer: Writer::new(),
            progress_bar: ProgressBar::new(NUM_GAMES),
            outcome: None,
        }
    }
}

impl<'a> Visitor<'a> for TestVisitor {
    fn start_game(&mut self) {
        self.chess = Board::default();
        self.writer = Writer::new();
        self.outcome = None
    }

    fn san(&mut self, san: SAN) {
        let chess_move = san.to_move(&self.chess);

        self.writer.add_move(&chess_move, &self.chess);
        self.chess = self.chess.make_move_new(chess_move);
    }

    fn end_game(&mut self, outcome: &str) {
        let mut reader = Reader::new(
            &self
                .writer
                .clone()
                .get_data(Some(Outcome::from_string(outcome))),
        );

        let (_, board) = reader.clone().last().unwrap();
        assert_eq!(self.chess, board);

        while let Some(_) = reader.next() {}

        assert_eq!(reader.get_outcome(), &Some(Outcome::from_string(outcome)));

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
