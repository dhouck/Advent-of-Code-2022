#![allow(incomplete_features)]
#![feature(adt_const_params)]

use aoc2022::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RPS {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
use RPS::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RoundState {
    Loss = 0,
    Tie = 3,
    Win = 6,
}

impl RPS {
    fn score(self) -> u32 {
        self as u32 + 1
    }

    fn versus(self, other: RPS) -> RoundState {
        if self == other {
            RoundState::Tie
        } else if self.loses_to() == other {
            RoundState::Loss
        } else {
            RoundState::Win
        }
    }

    fn loses_to(self) -> RPS {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }

    fn wins_to(self) -> RPS {
        self.loses_to().loses_to()
    }
}

#[derive(Copy, Clone, Debug)]
struct Round<const P: Part> {
    opponent: RPS,
    me: RPS,
}

impl FromStr for Round<{Part::A}> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((first, second)) = s.split_once(' ') else {
            return Err(anyhow!("Line contains no space character"))
        };

        let opponent = match first {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(anyhow!("Unknown opponent move")),
        };
        let me = match second {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => return Err(anyhow!("Unknown response move")),
        };

        Ok(Round {opponent, me})
    }
}

impl FromStr for Round<{Part::B}> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let Some((first, second)) = s.split_once(' ') else {
            return Err(anyhow!("Line contains no space character"))
        };

        let opponent = match first {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => return Err(anyhow!("Unknown opponent move")),
        };
        let me = match second {
            "X" => opponent.wins_to(),
            "Y" => opponent,
            "Z" => opponent.loses_to(),
            _ => return Err(anyhow!("Unknown response move")),
        };

        Ok(Round {opponent, me})
    }
}

impl<const P: Part> Round<P> {
    pub fn state(&self) -> RoundState {
        self.me.versus(self.opponent)
    }

    pub fn score(&self) -> u32 {
        self.me.score() + self.state() as u32
    }
}

fn part_a(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let total_score: u32 = input.lines()
        .map(|line| -> Result<u32> {
            Ok(line?.parse::<Round<{Part::A}>>()?.score())
        })
        .sum::<Result<u32>>()?;
    Ok(writeln!(output, "{}", total_score)?)
}

fn part_b(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let total_score: u32 = input.lines()
        .map(|line| -> Result<u32> {
            Ok(line?.parse::<Round<{Part::B}>>()?.score())
        })
        .sum::<Result<u32>>()?;
    Ok(writeln!(output, "{}", total_score)?)
}

fn main() -> Result<()> {
    let mut args = get_args()?;
    match args.part {
        Part::A => part_a(args.input, &mut args.output),
        Part::B => part_b(args.input, &mut args.output),
    }
}
