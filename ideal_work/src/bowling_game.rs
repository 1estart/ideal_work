#[cfg(test)]
mod bowling_game_tests {
    use super::*;

    #[test]
    fn can_create_game() {
        let game = BowlingGame::new();
        assert_eq!(game.score(), 0);
    }
}
