#![feature(iter_order_by)]

use aoc::grid::{chebyshev_ball, Grid};
use std::io;

fn main() -> io::Result<()> {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let grid = Grid::new(grid);

    let silver: usize = silver(&grid);
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn silver(grid: &Grid<char>) -> usize {
    grid.points()
        .flat_map(|p| chebyshev_ball(1).map(move |dp| grid.ray(p, dp)))
        .map(|cs| cs.take(4).eq_by("XMAS".chars(), |a, b| *a == b))
        .filter(|p| *p)
        .count()
}
