#[derive(Eq, PartialEq, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Shape {
    pub fn beats(&self, other: &Shape) -> bool {
        use Shape::*;
        matches!((self, other),
            (Rock, Scissors | Lizard)
            | (Paper, Rock | Spock)
            | (Scissors, Paper | Lizard)
            | (Lizard, Paper | Spock)
            | (Spock, Scissors | Rock)
        )
    }

    pub fn get_winner_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Spock,
            Shape::Spock => Shape::Lizard,
            Shape::Lizard => Shape::Rock,
        }
    }
}

pub fn parse_shape(shape: &str) -> Option<Shape> {
    match shape {
        "Rock" => Some(Shape::Rock),
        "Paper" => Some(Shape::Paper),
        "Scissors" => Some(Shape::Scissors),
        "Lizard" => Some(Shape::Lizard),
        "Spock" => Some(Shape::Spock),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn shape_comparison() {
        use super::*;

        assert!(Shape::Rock.beats(&Shape::Scissors));
        assert!(Shape::Paper.beats(&Shape::Spock));
        assert!(!Shape::Paper.beats(&Shape::Scissors));
        assert_eq!(Shape::Spock, Shape::Spock);
    }
}