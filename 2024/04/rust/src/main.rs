use aoc::Grid;
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
    let dirs = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    grid.points()
        .flat_map(|p| dirs.into_iter().map(move |dp| grid.ray(p, dp)))
        .map(|cs| cs.zip("XMAS".chars()).all(|(&a, b)| a == b))
        .filter(|p| *p)
        .count()
}
