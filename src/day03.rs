use std::collections::HashSet;
use crate::Result;

#[test]
pub fn day03() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.input")?;
    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<u32> {
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .map(|line| char_in_both_halves(line).unwrap())
        .map(to_prio)
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<u32> {
    let lines: Vec<_> = input.lines().collect();
    let (_, group_chars): (Vec<String>, Vec<char>) = lines
        .into_iter()
        .map(|s| s.to_owned())
        .fold((Vec::with_capacity(3), Vec::new()), triplet_fold);
    let sum = group_chars.iter().map(|c| to_prio(*c)).sum();
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

fn triplet_fold((mut collected_lines, mut common_chars) : (Vec<String>, Vec<char>), line: String) -> (Vec<String>, Vec<char>) {
    // let (mut collected_lines, mut common_chars) = acc;
    if collected_lines.len() < 3 {
        collected_lines.push(line);
        (collected_lines, common_chars)
    } else {
        let sets: Vec<HashSet<char>> = collected_lines
            .iter()
            .map(|s| s.chars().collect())
            .collect();
        let merged_set: HashSet<char> = sets
            .into_iter()
            .reduce(|set1, set2| {
                let intersection: HashSet<char> = set1.intersection(&set2).cloned().collect();
                intersection
            })
            .unwrap();
        merged_set
            .iter()
            .take(1)
            .for_each(|elem| common_chars.push(*elem));
        collected_lines.clear();
        (collected_lines, common_chars)
    }
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
