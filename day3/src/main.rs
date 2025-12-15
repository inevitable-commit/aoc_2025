use std::env;

use common::take_input;
use day3::{parse, part1, part2};

fn main() -> Result<(), String> {
    let part: u8 = env::args()
        .nth(1)
        .ok_or("Provide argument to specify which part to run")?
        .parse()
        .map_err(|_| "Couldn't parse")?;

    let texts = take_input("day3/input.txt");

    let result = if part == 1 {
        part1(parse(&texts))
    } else if part == 2 {
        part2(parse(&texts))
    } else {
        return Err("Enter 1 for part 1 or 2 for part 2".to_string());
    };

    println!("Answer = {result}");

    Ok(())
}
