use dialoguer::{Select, theme::ColorfulTheme, console::Term};

use crate::ascii::Hand;

pub enum GameResult {
    Draw,
    Win,
    Lose,
}

pub struct Game {
    pub hand: Hand,
    pub rival: Hand,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            hand: Hand::None,
            rival: Hand::None,
        }
    }
}

impl Game {
    pub fn hand(&mut self, hand: Hand) {
        self.hand = hand;
    }

    pub fn rival_hand(&mut self, hand: Hand) {
        self.rival = hand;
    }

    pub fn choice(&mut self) -> std::io::Result<()> {
        let items = vec!["paper", "scissors", "rock"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        println!("You choose {}, waited for the other side to choice", &selection.unwrap());
        let hand: Hand = match selection {
            Some(index) => match index {
                0 => Hand::Paper,
                1 => Hand::Scissors,
                2 => Hand::Rock,
                _ => Hand::None,
            },
            None => Hand::None
        };

        self.hand(hand);
        Ok(())
    }

    pub fn judge(&self) -> GameResult {
        if self.hand == Hand::None && self.rival == Hand::None {
            return GameResult::Draw;
        }

        if self.hand == Hand::None {
            return GameResult::Lose;
        }

        if self.rival == Hand::None {
            return GameResult::Win;
        }

        match ( &self.hand, &self.rival ) {
            (Hand::Scissors, Hand::Scissors) => GameResult::Draw,
            (Hand::Scissors, Hand::Rock) => GameResult::Lose,
            (Hand::Scissors, Hand::Paper) => GameResult::Win,
            (Hand::Scissors, Hand::None) => GameResult::Win,
            (Hand::Rock, Hand::Scissors) => GameResult::Lose,
            (Hand::Rock, Hand::Rock) => GameResult::Draw,
            (Hand::Rock, Hand::Paper) => GameResult::Lose,
            (Hand::Rock, Hand::None) => GameResult::Win,
            (Hand::Paper, Hand::Scissors) => GameResult::Lose,
            (Hand::Paper, Hand::Rock) => GameResult::Win,
            (Hand::Paper, Hand::Paper) => GameResult::Draw,
            (Hand::Paper, Hand::None) => GameResult::Win,
            (Hand::None, Hand::Scissors) => GameResult::Lose,
            (Hand::None, Hand::Rock) => GameResult::Lose,
            (Hand::None, Hand::Paper) => GameResult::Lose,
            (Hand::None, Hand::None) => GameResult::Lose,
        }
    }
}
