use std::{fmt, io};

#[derive(Eq, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Shape {
    fn gt(&self, other: &Shape) -> bool {
        use Shape::*;
        matches!((self, other),
            (Rock, Scissors | Lizard)
            | (Paper, Rock | Spock)
            | (Scissors, Paper | Lizard)
            | (Lizard, Paper | Spock)
            | (Spock, Scissors | Rock)
        )
    }

    fn get_winner_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Spock,
            Shape::Spock => Shape::Lizard,
            Shape::Lizard => Shape::Rock,
        }
    }
}

fn parse_shape(shape: &str) -> Option<Shape> {
    match shape {
        "Rock" => Some(Shape::Rock),
        "Paper" => Some(Shape::Paper),
        "Scissors" => Some(Shape::Scissors),
        "Lizard" => Some(Shape::Lizard),
        "Spock" => Some(Shape::Spock),
        _ => None
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Player {
    name: String,
    shape: Shape,
    score: u64,
}

impl Player {
    fn new(name: &str, shape: Shape) -> Self {
        Self {
            name: String::from(name),
            shape,
            score: 0,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Shape: {:?}, Score: {}", self.name, self.shape, self.score)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Game {
    player1: Player,
    player2: Player,
    turns: u64,
    ties: u64,
}

impl Game {
    fn new(instructions: &[String], player1_name: &str, player2_name: &str) -> Self {
        Self {
            player1: Player::new(player1_name, parse_shape(instructions[0].as_str()).unwrap()),
            player2: Player::new(player2_name, parse_shape(instructions[1].as_str()).unwrap()),
            turns: num_parse(&instructions[2]),
            ties: 0,
        }
    }
    fn in_loop(&mut self, turn_num: u64) -> bool {
        use Shape::*;
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

    fn run(&mut self) {
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
            } else if self.player1.shape.gt(&self.player2.shape) { // Player1 wins
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

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn num_parse(input: &str) -> u64 {
    input
        .trim()
        .parse()
        .expect("Failed to parse numeric string")
}

fn parse_game_line(game_line: &str, game_instructions: &mut Vec<String>) {
    game_instructions.extend(
        game_line
            .split(' ')
            .map(str::to_owned)
    )
}

fn game_loop(game_amount: u64) {
    let mut game_instructions: Vec<String> = Vec::with_capacity(3);
    for _game_num in 0..game_amount {
        game_instructions.clear();
        parse_game_line(&get_input(), &mut game_instructions);
        let mut game = Game::new(&game_instructions,
                                 "Alice",
                                 "Bob",
        );
        game.run();
        println!("{game}");
    }
}

fn main() {
    let game_amount = num_parse(&get_input());
    game_loop(game_amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_comparison() {
        assert!(Shape::Rock.gt(&Shape::Scissors));
        assert!(Shape::Paper.gt(&Shape::Spock));
        assert!(!Shape::Paper.gt(&Shape::Scissors));
        assert_eq!(Shape::Spock, Shape::Spock);
    }

    #[test]
    fn game_creation() {
        let baseline: Game = Game {
            player1: Player::new("player1", Shape::Spock),
            player2: Player::new("player2", Shape::Lizard),
            turns: 6,
            ties: 0,
        };

        let correct_game_instructions: Vec<String> = Vec::from(
            [
                String::from("Spock"),
                String::from("Lizard"),
                String::from("6")
            ]
        );

        let game_equal = Game::new(&correct_game_instructions, "player1", "player2");
        let wrong_game_instructions: Vec<String> = Vec::from(
            [
                String::from("Lizard"),
                String::from("Spock"),
                String::from("6")
            ]
        );

        let game_not_equal = Game::new(&wrong_game_instructions, "player1", "player2");

        assert_eq!(baseline, game_equal);
        assert_ne!(baseline, game_not_equal);
    }

    #[test]
    fn run_all_games() {
        use std::collections::HashMap;

        let mapping: HashMap<&str, &str> = HashMap::from(
            [
                ("Rock Rock 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
                ("Rock Paper 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Rock Scissors 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Rock Lizard 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Rock Spock 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Paper Rock 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Paper Paper 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
                ("Paper Scissors 25", "Bob wins, by winning 13 game(s) and tying 7 game(s)"),
                ("Paper Lizard 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Paper Spock 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Scissors Rock 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Scissors Paper 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Scissors Scissors 25", "Bob wins, by winning 12 game(s) and tying 8 game(s)"),
                ("Scissors Lizard 25", "Bob wins, by winning 12 game(s) and tying 6 game(s)"),
                ("Scissors Spock 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Lizard Rock 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Lizard Paper 25", "Alice wins, by winning 25 game(s) and tying 0 game(s)"),
                ("Lizard Scissors 25", "Bob wins, by winning 13 game(s) and tying 7 game(s)"),
                ("Lizard Lizard 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
                ("Lizard Spock 25", "Alice wins, by winning 25 game(s) and tying 0 game(s)"),
                ("Spock Rock 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
                ("Spock Paper 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Spock Scissors 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
                ("Spock Lizard 25", "Bob wins, by winning 13 game(s) and tying 6 game(s)"),
                ("Spock Spock 25", "Bob wins, by winning 12 game(s) and tying 7 game(s)"),
            ]
        );

        let mut parsed_instructions = Vec::with_capacity(3);
        let mut game: Game;
        for instruction in mapping.keys() {
            parsed_instructions.clear();
            parse_game_line(&String::from(*instruction), &mut parsed_instructions);
            game = Game::new(&parsed_instructions, "Alice", "Bob");
            game.run();
            assert_eq!(&format!("{}", game), mapping.get(instruction).unwrap());
        }
    }
}
