pub mod pieces;
pub mod board;
pub mod fen;

#[cfg(test)]
mod tests {
    #[test]
    fn should_fail() {
        assert_eq!(false, true);
    }
}