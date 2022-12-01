mod path_or_std;
pub mod prelude;

use anyhow::Result;
use clap::{ValueEnum, Parser};
use path_or_std::{PathOrStd, InputFile, OutputFile};

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum Part {
    A,
    B,
}

#[derive(Parser, Debug)]
#[clap(about, author, version)]
struct CLIArgs {
    #[arg(value_enum, default_value_t = Part::A)]
    part: Part,
    #[arg(default_value_t)]
    input: PathOrStd,
    #[arg(long, short, default_value_t)]
    output: PathOrStd,
}

#[derive(Debug)]
pub struct Args {
    pub part: Part,
    pub input: InputFile,
    pub output: OutputFile,
}

impl TryFrom<CLIArgs> for Args {
    type Error = std::io::Error;

    fn try_from(cli_args: CLIArgs) -> Result<Self, Self::Error> {
        Ok(Args {
            part: cli_args.part,
            input: cli_args.input.open()?,
            output: cli_args.output.create()?
        })
    }
}

pub fn get_args() -> anyhow::Result<Args> {
    Ok(Args::try_from(CLIArgs::parse())?)
}