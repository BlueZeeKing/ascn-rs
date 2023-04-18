# ascn-rs

Assumed Standard Chess Notation is a format for encoding the entire history of a chess game in as small a space as possible in reasonable time. This algorithm stores each move in about a byte and occasionally overflows to handle edge cases. Made for the AP Computer Science Principles Create Task.

## Dependencies

This project uses [shakmaty](https://crates.io/crates/shakmaty) and [pgn-reader](https://crates.io/crates/pgn-reader) which are both licensed under GPLv3.
