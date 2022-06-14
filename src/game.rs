use std::fmt;

use crate::{Player, shape, Shape};

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub turns: u64,
    pub ties: u64,
}

impl Game {
    pub fn new(instructions: &[String], player1_name: &str, player2_name: &str) -> Self {
        let player1_shape = shape::parse_shape(instructions[0].as_str()).expect("Could not parse shape");
        let player2_shape = shape::parse_shape(instructions[1].as_str()).expect("Could not parse shape");
        Self {
            player1: Player::new(player1_name, player1_shape),
            player2: Player::new(player2_name, player2_shape),
            turns: crate::utils::num_parse(&instructions[2]),
            ties: 0,
        }
    }

    fn in_loop(&mut self, turn_num: u64) -> bool {
        use crate::Shape::*;
        let turns_left = self.turns - turn_num;
        match (&self.player1.shape, &self.player2.shape) {
            (Lizard, Spock) | (Lizard, Paper) => {
                self.player1.score += turns_left;
                true
            }
            (Paper, Paper) | (Paper, Spock) | (Scissors, Spock) | (Lizard, Rock) => {
                if turns_left % 4 == 0 {
                    self.player1.score += turns_left / 4;
                    self.player2.score += turns_left / 2;
                    self.ties += turns_left / 4;
                    true
                } else {
                    false
                }
            }
            _ => false
        }
    }

    pub fn run(&mut self) {
        for turn_num in 0..self.turns {
            if self.in_loop(turn_num) {
                break;
            }
            if self.player1.shape == self.player2.shape { // Tie
                self.ties += 1;
                self.player1.shape = self.player1.shape.get_winner_shape(); // Alice rule 2
                if self.player2.shape == Shape::Spock {
                    self.player2.shape = Shape::Lizard; // Bob rule 3
                } else {
                    self.player2.shape = Shape::Spock; // Bob rule 1
                }
            } else if self.player1.shape.beats(&self.player2.shape) { // Player1 wins
                self.player1.score += 1;
                if self.player2.shape == Shape::Spock {
                    self.player2.shape = Shape::Paper; // Bob rule 4
                } else {
                    self.player2.shape = Shape::Spock; // Bob rule 1
                }
            } else { // Player2 wins
                self.player2.score += 1;
                self.player1.shape = self.player2.shape.get_winner_shape(); // Alice rule 3
                if self.player2.shape == Shape::Spock {
                    self.player2.shape = Shape::Rock; // Bob rule 2
                } else {
                    self.player2.shape = Shape::Spock; // Bob rule 1
                }
            }
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.player1.score, self.player2.score) {
            (x, y) if x == y => {
                write!(f, "{} and {} tie, each winning {} game(s) and tying {} game(s)",
                       self.player1.name,
                       self.player2.name,
                       self.player1.score,
                       self.ties
                )
            }
            (x, y) if x > y => {
                write!(f, "{} wins, by winning {} game(s) and tying {} game(s)",
                       self.player1.name,
                       self.player1.score,
                       self.ties
                )
            }
            _ => {
                write!(f, "{} wins, by winning {} game(s) and tying {} game(s)",
                       self.player2.name,
                       self.player2.score,
                       self.ties
                )
            }
        }
    }
}

pub fn parse_game_line(game_line: &str, game_instructions: &mut Vec<String>) {
    game_instructions.extend(
        game_line
            .split(' ')
            .map(str::to_owned)
    )
}

pub fn game_loop(game_amount: u64) {
    let mut game_instructions: Vec<String> = Vec::with_capacity(3);
    for _game_num in 0..game_amount {
        game_instructions.clear();
        parse_game_line(&crate::utils::get_input(), &mut game_instructions);
        let mut game = Game::new(&game_instructions,
                                 "Alice",
                                 "Bob",
        );
        game.run();
        println!("{game}");
    }
}
