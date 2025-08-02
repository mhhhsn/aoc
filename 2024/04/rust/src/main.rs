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
    let gold: usize = gold(&grid);
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

fn gold(grid: &Grid<char>) -> usize {
    let xmas: Vec<(i64, i64)> = vec![(0, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)];
    grid.points()
        .map(move |p| {
            xmas.iter()
                .map(|&d| grid.move_pos(p, d.into(), 1).and_then(|p| grid.at(p)))
                .collect::<Option<Vec<_>>>()
        })
        .flatten()
        .map(|cs| {
            vec!["AMSMS", "AMMSS", "ASMSM", "ASSMM"]
                .iter()
                .map(|x| cs.iter().eq_by(x.chars(), |&&a, b| a == b))
                .any(|p| p)
        })
        .filter(|p| *p)
        .count()
}
