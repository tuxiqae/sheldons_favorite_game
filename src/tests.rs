#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::shape::Shape;
    use crate::player::Player;

    #[test]
    fn shape_comparison() {
        assert!(Shape::Rock.beats(&Shape::Scissors));
        assert!(Shape::Paper.beats(&Shape::Spock));
        assert!(!Shape::Paper.beats(&Shape::Scissors));
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
        use crate::game::parse_game_line;

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
