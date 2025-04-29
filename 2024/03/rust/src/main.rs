use regex::{Captures, Regex};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut program = String::new();
    io::stdin().read_to_string(&mut program)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let silver: u64 = re
        .captures_iter(&program)
        .map(self::execute_mul_capture)
        .sum();

    let gold_re = Regex::new(r"(mul)\(\d{1,3},\d{1,3}\)|(do)\(\)|(don't)\(\)").unwrap();

    let mut captures = gold_re.captures_iter(&program).map(|c| c.extract::<1>());

    let mut gold: u64 = 0;
    while let Some((func, [name])) = captures.next() {
        match name {
            "mul" => gold += execute_mul_capture(re.captures(func).unwrap()),
            "do" => (),
            "don't" => {
                while let Some((_, [n])) = captures.next() {
                    if n == "do" {
                        break;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn execute_mul_capture(captures: Captures) -> u64 {
    let (_, [x, y]) = captures.extract();
    x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap()
}
