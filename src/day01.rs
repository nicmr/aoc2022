use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn day01() -> Result<()> {
    let input = std::fs::read_to_string("input/day01.input")?;
    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<i64> {
    let max_sum = parse_elf_calories(input)?.into_iter().max();
    match max_sum {
        Some(max) => Ok(max),
        None => Err("Can't find max: empty input".into()),
    }
}

pub fn part2(input: &str) -> Result<i64> {
    let mut calories = parse_elf_calories(input)?;
    calories.sort_unstable();
    let sum_top_3 = calories.iter().rev().take(3).sum();
    Ok(sum_top_3)
}

pub fn parse_elf_calories(input: &str) -> Result<Vec<i64>> {
    let lines: Vec<_> = input.lines().collect();
    Ok(lines
        .split(|line| line == &"")
        .map(|linegroup| {
            linegroup
                .iter()
                .map(|elem| str::parse::<i64>(elem).unwrap())
                .sum()
        })
        .collect())
}
