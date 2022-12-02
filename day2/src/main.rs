use std::{str::FromStr, time::Instant};

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Tie = 3,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn wins_against(self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }
    fn loses_against(self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
    fn fight(self, other: Self) -> Outcome {
        if self.wins_against() == other {
            Outcome::Win
        } else if self == other {
            Outcome::Tie
        } else {
            Outcome::Loss
        }
    }
    fn get_opponent_for_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.loses_against(),
            Outcome::Tie => self,
            Outcome::Loss => self.wins_against(),
        }
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

struct RoundPart1 {
    me: Choice,
    opponent: Choice,
}

impl RoundPart1 {
    fn get_score(&self) -> u32 {
        self.me.fight(self.opponent) as u32 + self.me as u32
    }
}

struct RoundPart2 {
    opponent: Choice,
    outcome: Outcome,
}

impl RoundPart2 {
    fn get_score(&self) -> u32 {
        self.outcome as u32 + self.opponent.get_opponent_for_outcome(self.outcome) as u32
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("input");

    let rounds: Vec<(RoundPart1, RoundPart2)> = input
        .lines()
        .map(|l| {
            let split = l.split_once(' ').unwrap();
            (
                RoundPart1 {
                    me: split.1.parse().unwrap(),
                    opponent: split.0.parse().unwrap(),
                },
                RoundPart2 {
                    opponent: split.0.parse().unwrap(),
                    outcome: split.1.parse().unwrap(),
                },
            )
        })
        .collect();

    let total_1: u32 = rounds.iter().map(|r| r.0.get_score()).sum();

    let total_2: u32 = rounds.iter().map(|r| r.1.get_score()).sum();

    let end = Instant::now();
    println!("Time: {:#?}", end - start);

    println!("Part 1: {total_1}");
    println!("Part 2: {total_2}");
}
