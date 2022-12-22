use crate::Result;
use crate::BoxedError;

#[test]
pub fn day04() -> Result<()> {
    let input = std::fs::read_to_string("input/day04.input")?;
    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct Range {
    start: u32,
    end: u32,
}

pub fn part1(input: &str) -> Result<u32> {
    let result =
    input.lines()
                .map(parse_ranges)
                .map(|a| a.unwrap())
                .map(check_line_for_superset)
                .filter(|b| *b)
                // .zip(input.lines())
                // .map(|(is_superset, line)|{
                //     println!("line: {} superset: {}", line, is_superset);
                //     is_superset
                // })
                .count();
    Ok(result as u32)
}

pub fn part2(input: &str) -> Result<u32> {
    let result =
        input.lines()
            .map(parse_ranges)
            .map(|result| result.unwrap())
            .map(|(left, right)| left.overlaps(&right))
            .filter(|b| *b)
            .count();
    Ok(result as u32)
}

pub fn parse_ranges(line: &str) -> Result<(Range, Range)> {
    let mut substrings = line.split(',');
    let left: Range = substrings.next().ok_or("Failed to get string left of comma")?.try_into()?;
    let right: Range = substrings.next().ok_or("Failed to get string right of comma")?.try_into()?;
    Ok((left, right))
}

pub fn check_line_for_superset((left, right) : (Range, Range)) -> bool {
    left.contains(&right) || right.contains(&left)
}

impl TryFrom<&str> for Range 
where
{
    type Error = BoxedError;

    fn try_from(s: &str) -> Result<Self> {
        let input_too_short = "Input too short";
        let mut parts = s.split('-');
        let start = parts.next().ok_or(input_too_short)?.parse::<u32>()?;
        let end = parts.next().ok_or(input_too_short)?.parse::<u32>()?;
        let range = Self {
            start,
            end
        };
        // println!("Parsed {} as {:?}", s, range);        
        Ok(range)
    }
}

impl Range {
    /// Returns whether self contains `other`
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        (self.end >= other.start && self.start <= other.start) || (other.start <= self.start && other.end >= self.start)
    }
}

#[cfg(test)]
mod tests {
    use crate::Result;


    fn get_example() -> String {
        let example_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        example_input.to_owned()
    }

    #[test]
    fn day04_example01() -> Result<()>{
        let input = get_example();
        let result = super::part1(&input)?;
        assert_eq!(2, result);
        return Ok(())
    }

    #[test]
    fn day04_example02() -> Result<()>{
        let input = get_example();
        let result = super::part2(&input)?;
        assert_eq!(4, result);
        return Ok(())
    }
}