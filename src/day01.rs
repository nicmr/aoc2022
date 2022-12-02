use std::{error::Error, fs};

pub fn part1() -> Result<i64, Box<dyn Error>> {
    let max_sum = parse_elf_calories()?.into_iter().max();
    match max_sum {
        Some(max) => Ok(max),
        None => Err("Can't find max: empty input".into()),
    }
}

pub fn part2() -> Result<i64, Box<dyn Error>> {
    let mut calories = parse_elf_calories()?;
    calories.sort_unstable();
    let sum_top_3 = calories.iter().rev().take(3).sum();
    Ok(sum_top_3)
}

pub fn parse_elf_calories() -> Result<Vec<i64>, Box<dyn Error>> {
    let input = fs::read_to_string("day01.input")?;
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
