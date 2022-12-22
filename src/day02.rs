use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
pub fn day02() -> Result<()> {
    let input = std::fs::read_to_string("input/day02.input")?;
    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);
    Ok(())
}

pub fn part1(input: &str) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .map(|line| Match::try_from_str(line).unwrap())
        .map(|mtch| mtch.get_player_score())
        .sum();
    Ok(sum)
}

pub fn part2(input: &str) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let sum = lines
        .iter()
        .map(|line| Match::try_from_str_part2(line).unwrap())
        .map(|mtch| mtch.get_player_score())
        .sum();
    Ok(sum)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn try_from_char_enemy(c: char) -> Result<Self> {
        match c {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err("character has no corresponding enum variant".into()),
        }
    }
    fn try_from_char_player(c: char) -> Result<Self> {
        match c {
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
            _ => Err("character has no corresponding enum variant".into()),
        }
    }
}

/// Describes the Result of a rock, paper, scissors game from the player perspective
enum MatchResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl MatchResult {
    fn try_from_char(c: char) -> Result<Self> {
        match c {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err("character has no corresponding enum variant".into()),
        }
    }
}

struct Match {
    player: Move,
    enemy: Move,
}

impl Match {
    fn from_enemy_and_result(enemy: Move, desired_result: MatchResult) -> Self {
        let player = match (enemy, desired_result) {
            (Move::Rock, MatchResult::Win) => Move::Paper,
            (Move::Rock, MatchResult::Loss) => Move::Scissors,
            (Move::Paper, MatchResult::Win) => Move::Scissors,
            (Move::Paper, MatchResult::Loss) => Move::Rock,
            (Move::Scissors, MatchResult::Win) => Move::Rock,
            (Move::Scissors, MatchResult::Loss) => Move::Paper,
            (e, MatchResult::Draw) => e,
        };
        Self { player, enemy }
    }

    fn try_from_str(s: &str) -> Result<Self> {
        let enemy_char: char = s.chars().nth(0).unwrap();
        // .ok_or(Err("Failed to find 0th element in line".into()))?;
        let enemy_move = Move::try_from_char_enemy(enemy_char)?;

        let player_char: char = s.chars().nth(2).unwrap();
        // .ok_or(Err("Failed to find second element in line".into()))?;
        let player_move = Move::try_from_char_player(player_char)?;
        Ok(Self {
            player: player_move,
            enemy: enemy_move,
        })
    }

    fn try_from_str_part2(s: &str) -> Result<Self> {
        let enemy_char = s.chars().nth(0).unwrap();
        let enemy_move = Move::try_from_char_enemy(enemy_char)?;
        let desired_result_char = s.chars().nth(2).unwrap();
        let desired_result = MatchResult::try_from_char(desired_result_char)?;
        Ok(Self::from_enemy_and_result(enemy_move, desired_result))
    }

    fn get_match_result(&self) -> MatchResult {
        match (self.player, self.enemy) {
            (Move::Scissors, Move::Paper) => MatchResult::Win,
            (Move::Rock, Move::Scissors) => MatchResult::Win,
            (Move::Paper, Move::Rock) => MatchResult::Win,
            (a, b) if a == b => MatchResult::Draw,
            _ => MatchResult::Loss,
        }
    }

    fn get_player_score(&self) -> i32 {
        self.player as i32 + self.get_match_result() as i32
    }
}
