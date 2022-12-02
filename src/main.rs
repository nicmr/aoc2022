use std::error::Error;

mod day01;

fn main() -> Result<(), Box<dyn Error>> {
    let part1_result = day01::part1()?;
    println!("part 1 result: {}", part1_result);
    let part2_result = day01::part2()?;
    println!("part 2 result: {}", part2_result);
    Ok(())
}
