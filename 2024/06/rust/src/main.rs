use std::io;

use aoc::grid::{Direction, Grid};
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let grid: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let grid = Grid::new(grid);
    let start = grid
        .enumerate()
        .filter(|(_, c)| **c == '^')
        .next()
        .unwrap()
        .0;

    let silver: usize = silver(&grid, start);
    let gold: usize = gold(&grid);
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn silver(grid: &Grid<char>, start: (usize, usize)) -> usize {
    let mut pos = start;
    let mut dir = Direction { x: 0, y: -1 };
    let mut seen = HashSet::<(usize, usize)>::new();
    loop {
        seen.insert(pos);
        match grid.move_pos(pos, dir, 1).and_then(|p| grid.at(p)) {
            Some(&'#') => dir = dir.cw(),
            Some(_) => pos = grid.move_pos(pos, dir, 1).unwrap(),
            None => break seen.len(),
        }
    }
}

fn gold(grid: &Grid<char>) -> usize {
    0
}
