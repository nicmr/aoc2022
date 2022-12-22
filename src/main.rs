use std::error::Error;

type BoxedError = Box<dyn Error>;
type Result<T> = std::result::Result<T, BoxedError>;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// pub mod day10;

fn main() -> Result<()> {
    Ok(())
}
