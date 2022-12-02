use aoc2022::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
use RPS::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RoundState {
    Loss = 0,
    Tie = 3,
    Win = 6,
}

impl RPS {
    fn versus(self, other: RPS) -> RoundState {
        if self == other {
            RoundState::Tie
        } else if ((self as u32) % 3) + 1 == (other as u32) {
            RoundState::Loss
        } else {
            RoundState::Win
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Round {
    opponent: RPS,
    me: RPS,
}

impl FromStr for Round {
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

impl Round {
    pub fn state(&self) -> RoundState {
        self.me.versus(self.opponent)
    }

    pub fn score(&self) -> u32 {
        self.me as u32 + self.state() as u32
    }
}

fn part_a(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let total_score: u32 = input.lines()
        .map(|line| -> Result<u32> {
            Ok(line?.parse::<Round>()?.score())
        })
        .sum::<Result<u32>>()?;
    Ok(writeln!(output, "{}", total_score)?)
}

fn part_b(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let _ = (input, output);
    unimplemented!()
}

fn main() -> Result<()> {
    let mut args = get_args()?;
    match args.part {
        Part::A => part_a(args.input, &mut args.output),
        Part::B => part_b(args.input, &mut args.output),
    }
}
