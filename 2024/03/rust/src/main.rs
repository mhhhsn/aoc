use regex::Regex;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut program = String::new();
    io::stdin().read_to_string(&mut program)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let silver: u64 = re
        .captures_iter(&program)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap())
        .sum();
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
