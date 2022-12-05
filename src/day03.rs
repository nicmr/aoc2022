use std::{collections::HashSet, error::Error};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
pub fn day03() -> Result<()> {
    let input = std::fs::read_to_string("day03.input")?;
    println!("part1: {}", part1(&input)?);
    // println!("part2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<u32> {
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .map(|line| char_in_both_halves(line).unwrap())
        .map(|c| to_prio(c))
        .sum();
    Ok(sum)
}

fn char_in_both_halves(line: &str) -> Option<char> {
    let left_set: HashSet<char> = line.chars().take(line.len() / 2).collect();
    for right_elem in line.chars().skip(line.len() / 2) {
        if let Some(char_in_both) = left_set.get(&right_elem) {
            return Some(*char_in_both);
        }
    }
    None
}

fn to_prio(c: char) -> u32 {
    if c as u32 > 96 {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_prio() {
        assert_eq!(16u32, to_prio('p'));
        assert_eq!(38u32, to_prio('L'));
        assert_eq!(42u32, to_prio('P'));
        assert_eq!(22u32, to_prio('v'));
    }
    #[test]
    fn test_char_in_both_halves() {
        assert_eq!('a', char_in_both_halves("123a456a").unwrap());
    }
}

// pub fn part2(input: &str) -> Result<i32> {
//     let lines: Vec<_> = input.lines().collect();
//     let sum = lines
//         .iter()
//         .map(|line| Match::try_from_str_part2(line).unwrap())
//         .map(|mtch| mtch.get_player_score())
//         .sum();
//     Ok(sum)
// }
