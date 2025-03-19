use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let (mut ls, mut rs): (Vec<u64>, Vec<u64>) = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>()
        })
        .map(|xs| (xs[0], xs[1]))
        .unzip();

    ls.sort_unstable();
    rs.sort_unstable();

    let mut counts = HashMap::new();
    rs.iter().for_each(|&x| {
        *counts.entry(x).or_insert(0) += 1;
    });

    let silver: u64 = ls.iter().zip(rs).map(|(x, y)| x.abs_diff(y)).sum();
    let gold: u64 = ls.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum();

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
