#![feature(iter_map_windows)]

use std::io;

fn main() {
    let input: Vec<Vec<i64>> = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(str::parse::<i64>)
                .collect()
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let silver: usize = input.iter().filter(|&xs| all_safe(xs)).count();
    let gold: usize = input.iter().filter(|&xs| drop_safe(xs)).count();
    println!("silver: {silver}");
    println!("gold: {gold}");
}

fn all_safe(xs: &Vec<i64>) -> bool {
    let mut diffs = xs.iter().map_windows(|&[x, y]| y - x).peekable();
    let dir = diffs.peek().unwrap_or(&1).signum();
    diffs.all(|d| (1..=3).contains(&d.abs()) && d.signum() == dir)
}

fn drop_safe(xs: &Vec<i64>) -> bool {
    (0..xs.len())
        .map(|idx| xs.split_at(idx))
        .any(|(left, right)| all_safe(&[left, &right[1..]].concat()))
}
