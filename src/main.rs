use std::{fmt, io};

#[derive(Hash, Eq, PartialEq, Debug)]
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
        match (self, other) {
            (Rock, Scissors | Lizard) => true,
            (Paper, Rock | Spock) => true,
            (Scissors, Paper | Lizard) => true,
            (Lizard, Paper | Spock) => true,
            (Spock, Scissors | Rock) => true,
            _ => false
        }
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

#[derive(Debug)]
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


#[derive(Debug)]
struct Game {
    player1: Player,
    player2: Player,
    turns: u64,
    ties: u64,
}

impl Game {
    fn in_loop(&mut self, turn_num: u64) -> bool {
        use Shape::*;
        let turns_left = self.turns - turn_num;
        return match (&self.player1.shape, &self.player2.shape) {
            (Lizard, Spock) | (Lizard, Paper) => {
                self.player1.score = turns_left;
                true
            }
            (Paper, Paper) | (Paper, Spock) | (Scissors, Spock) | (Lizard, Rock) => {
                if turns_left % 4 == 0 {
                    self.player1.score += turns_left / 2;
                    self.player2.score += turns_left / 4;
                    self.ties += turns_left / 4;
                    true
                } else {
                    false
                }
            }
            _ => false
        };
    }

    fn play(&mut self) {
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
        println!("{}", self);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.player1.score == self.player2.score {
            write!(f, "{} and {} tie, each winning {} game(s) and tying {} game(s)",
                   self.player1.name,
                   self.player2.name,
                   self.player1.score,
                   self.ties
            )
        } else if self.player1.score > self.player2.score {
            write!(f, "{} wins, by winning {} game(s) and tying {} game(s)",
                   self.player1.name,
                   self.player1.score,
                   self.ties
            )
        } else {
            write!(f, "{} wins, by winning {} game(s) and tying {} game(s)",
                   self.player2.name,
                   self.player2.score,
                   self.ties
            )
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

fn num_parse(input: String) -> u64 {
    input
        .trim()
        .parse()
        .expect("Failed to parse numeric string")
}

fn get_game_amount() -> u64 {
    num_parse(get_input())
}

fn parse_instructions(instructions: String) -> Game {
    let parsed: Vec<&str> = instructions.split(' ').collect();
    Game {
        player1: Player::new("Alice", parse_shape(parsed[0]).unwrap()),
        player2: Player::new("Bob", parse_shape(parsed[1]).unwrap()),
        turns: num_parse(String::from(parsed[2])),
        ties: 0,
    }
}

fn game_loop() {
    for _game_num in 1..=get_game_amount() {
        let mut game = parse_instructions(get_input());

        game.play();
    }
}

fn main() {
    game_loop();

    // Tests
    assert!(Shape::Rock.gt(&Shape::Scissors));
    assert!(Shape::Paper.gt(&Shape::Spock));
    assert!(!Shape::Paper.gt(&Shape::Scissors));
}