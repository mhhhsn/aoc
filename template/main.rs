use std::io;

fn main() {
    let lines = io::stdin()
        .lines()
        .flatten()
        .map(|line| line)
        .collect::<Vec<_>>();

    dbg!(lines);

    let silver: u64 = 0;
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");
}
