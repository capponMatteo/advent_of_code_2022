use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Shape {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn get_strategy_one() -> Vec<(Shape, Shape)> {
    let file = File::open("./inputs/day_two.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let opponent_move = line.chars().nth(0).unwrap();
            let my_move = line.chars().nth(2).unwrap();
            (
                opponent_move.try_into().unwrap(),
                my_move.try_into().unwrap(),
            )
        })
        .collect()
}

fn get_strategy_two() -> Vec<(Shape, Outcome)> {
    let file = File::open("./inputs/day_two.txt").unwrap();
    BufReader::new(file)
        .lines()
        .map(|l| {
            let line = l.unwrap();
            let opponent_move = line.chars().nth(0).unwrap();
            let expected_outcome = line.chars().nth(2).unwrap();
            (
                opponent_move.try_into().unwrap(),
                expected_outcome.try_into().unwrap(),
            )
        })
        .collect()
}

fn get_score_one(round: &(Shape, Shape)) -> i32 {
    use Shape::*;
    let result_score = match round {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,
    };
    result_score + round.1.score()
}

fn get_score_two(round: &(Shape, Outcome)) -> i32 {
    use Outcome::*;
    use Shape::*;
    let my_move = match round {
        (Rock, outcome) => match outcome {
            Win => Paper,
            Draw => Rock,
            Loss => Scissors,
        },
        (Paper, outcome) => match outcome {
            Win => Scissors,
            Draw => Paper,
            Loss => Rock,
        },
        (Scissors, outcome) => match outcome {
            Win => Rock,
            Draw => Scissors,
            Loss => Paper,
        },
    };
    my_move.score() + round.1.score()
}

#[allow(dead_code)]
fn part_one() -> i32 {
    let strategy = get_strategy_one();
    strategy.iter().map(|round| get_score_one(round)).sum()
}

#[allow(dead_code)]
fn part_two() -> i32 {
    let strategy = get_strategy_two();
    strategy.iter().map(|round| get_score_two(round)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 12276);
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 9975);
    }
}
