pub struct BowlingGame {
    rolls: [u32; 21],
    current_roll: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: [0; 21],
            current_roll: 0,
        }
    }

    fn strike_bonus(&self, roll_index: usize) -> u32 {
        self.rolls[roll_index + 1] + self.rolls[roll_index + 2]
    }

    fn spare_bonus(&self, roll_index: usize) -> u32 {
        self.rolls[roll_index + 2]
    }

    fn two_balls_in_frame(&self, roll_index: usize) -> u32 {
        self.rolls[roll_index] + self.rolls[roll_index + 1]
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        let mut roll_index = 0;
        for _frame_index in 0..10 {
            if self.is_strike(roll_index) {
                score += 10 + self.strike_bonus(roll_index);
                roll_index += 1;
                continue;
            } else if self.is_spare(roll_index) {
                score += 10 + self.spare_bonus(roll_index);
            } else {
                score += self.two_balls_in_frame(roll_index)
            }
            roll_index += 2;
        }
        score
    }

    fn is_spare(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] + self.rolls[roll_index + 1] == 10
    }

    fn is_strike(&self, roll_index: usize) -> bool {
        self.rolls[roll_index] == 10
    }

    pub fn roll(&mut self, pins: u32) {
        self.rolls[self.current_roll] = pins;
        self.current_roll += 1;
    }

    pub fn roll_many(&mut self, rolls: usize, pins: u32) {
        for _ in 0..rolls {
            self.roll(pins);
        }
    }

    pub fn roll_spare(&mut self) {
        self.roll_many(2, 5);
    }

    pub fn roll_strike(&mut self) {
        self.roll(10);
    }
}
#[cfg(test)]
mod bowling_game_tests {
    use super::*;

    #[test]
    fn worst_game() {
        let mut game = BowlingGame::new();
        game.roll_many(20, 0);
        assert_eq!(game.score(), 0);
    }

    #[test]
    fn all_ones() {
        let mut game = BowlingGame::new();
        game.roll_many(20, 1);
        assert_eq!(game.score(), 20);
    }

    #[test]
    fn spare() {
        let mut game = BowlingGame::new();
        game.roll_spare();
        game.roll(3);
        game.roll_many(17, 0);
        assert_eq!(game.score(), 16);
    }

    #[test]
    fn one_strike() {
        let mut game = BowlingGame::new();
        game.roll_strike();
        game.roll(3);
        game.roll(4);
        game.roll_many(16, 0);
        assert_eq!(game.score(), 24);
    }
    #[test]
    fn perfect_game() {
        let mut game = BowlingGame::new();
        game.roll_many(12, 10);
        assert_eq!(game.score(), 300);
    }
}
