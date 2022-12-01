use aoc2022::prelude::*;

type Calories = u32;

fn parse_input(input: impl BufRead) -> Result<Vec<Vec<Calories>>> {
    let mut output: Vec<Vec<Calories>> = vec![default()];
    for line in input.lines() {
        let line = line?;
        if line.len() == 0 {
            output.push(default());
            continue;
        }

        output.last_mut().ok_or(anyhow!("Should be impossible"))?.push(line.parse()?)
    }

    Ok(output)
}

fn part_a(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let elves = parse_input(input)?;
    // println!("{:?}", elves);
    let max_calories = elves.iter()
        .map(|l| l.iter().sum::<Calories>())
        .max()
        .ok_or(anyhow!("No elves listed"))?;
    writeln!(output, "{}", max_calories)?;
    Ok(())
}

fn part_b(input: impl BufRead, output: &mut OutputFile) -> Result<()> {
    let elves = parse_input(input)?;
    // println!("{:?}", elves);
    let mut sorted: Vec<Calories> = elves.iter()
        .map(|l| l.iter().sum::<Calories>())
        .collect();
    sorted.sort_by_key(|&calories|std::cmp::Reverse(calories));
    let top_3_calories: Calories = sorted[..3].iter().sum();
    writeln!(output, "{}", top_3_calories)?;
    Ok(())
}

fn main() -> Result<()> {
    let mut args = get_args()?;
    match args.part {
        Part::A => part_a(args.input, &mut args.output),
        Part::B => part_b(args.input, &mut args.output),
    }
}
