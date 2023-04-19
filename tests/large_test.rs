// use std::fs::{read_to_string, File};

// use ascn_rs::{reader::Reader, writer::Writer};
// use chess::{Board, ChessMove};
// use indicatif::ProgressIterator;
// use regex::Regex;

// // use ascn_rs::{reader::Reader, writer::Writer};
// // use chess::Board;
// // use indicatif::ProgressBar;
// // use pgn_reader::{BufferedReader, Visitor};

// const NUM_GAMES: u64 = 121332;

// // struct TestVisitor {
// //     chess: Board,
// //     writer: Writer,
// //     progress_bar: ProgressBar,
// // }

// // impl TestVisitor {
// //     fn new() -> Self {
// //         Self {
// //             chess: Board::default(),
// //             writer: Writer::new(),
// //             progress_bar: ProgressBar::new(NUM_GAMES),
// //         }
// //     }
// // }

// // impl Visitor for TestVisitor {
// //     fn begin_game(&mut self) {
// //         self.chess = Board::default();
// //         self.writer = Writer::new();
// //     }

// //     fn san(&mut self, _san_plus: pgn_reader::SanPlus) {
// //         let chess_move = _san_plus.san.to_move(&self.chess).unwrap();

// //         self.writer.add_move(&chess_move, &self.chess);
// //         self.chess = self.chess.clone().play(&chess_move).unwrap();
// //     }

// //     type Result = ();

// //     fn end_game(&mut self) -> Self::Result {
// //         let (_chess_move, game) = Reader::new(&self.writer.clone().get_data()).last().unwrap();
// //         assert_eq!(game, self.chess);
// //         self.progress_bar.inc(1);
// //     }
// // }

// #[test]
// #[ignore]
// fn test() {
//     let re = Regex::new(r"(?:.+\n)+\n(.+?)\d+?-\d+?").unwrap();
//     let individual_re = Regex::new(r"\d+. (.+?) (.+?) ").unwrap();

//     let data = read_to_string("tests/lichess_test_collection.pgn").unwrap();

//     for (index, game) in re
//         .captures_iter(&data)
//         .progress_count(NUM_GAMES)
//         .enumerate()
//     {
//         let data = game.get(1).unwrap().as_str().trim().to_string();

//         let mut writer = Writer::new();
//         let mut board = Board::default();

//         for (_, chess_move_str) in data
//             .split(" ")
//             .enumerate()
//             .filter(|(index, _)| index % 3 != 0)
//         {
//             if index == 12 {
//                 dbg!((format!("{}", board), chess_move_str));
//             }
//             let chess_move = ChessMove::from_san(&board, chess_move_str).unwrap();

//             writer.add_move(&chess_move, &board);
//             board = board.make_move_new(chess_move);
//         }

//         let (_, final_board) = Reader::new(&writer.get_data()).last().unwrap();

//         assert_eq!(board, final_board);
//     }

//     // let mut visitor = TestVisitor::new();
//     // let mut reader = BufferedReader::new(File::open("tests/lichess_test_collection.pgn").unwrap());

//     // reader.read_all(&mut visitor).unwrap();
// }
