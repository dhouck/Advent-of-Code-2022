use aoc2022::prelude::*;

fn part_a(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let _ = (input, output);
    unimplemented!()
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
