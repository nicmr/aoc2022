use std::{env, process::Command, error::Error};

type BoxedError = Box<dyn Error>;
type Result<T> = std::result::Result<T, BoxedError>;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
// pub mod day10;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let day_arg = args.get(1).ok_or("Missing argument. The argument should be the day")?;
    let cmd = Command::new("cargo")
        .arg("test")
        .arg(day_arg)
        .args(["--", "--nocapture"])
        .output()?;
    println!("{}", String::from_utf8_lossy(&cmd.stdout));
    Ok(())
}
