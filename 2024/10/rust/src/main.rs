use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let grid: Vec<Vec<u8>> = io::stdin()
        .lines()
        .flatten()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    let starts = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, d)| (x, y, d)))
        .filter_map(|(x, y, d)| (*d == 0).then_some((y, x)))
        .collect::<Vec<_>>();

    let silver: usize = starts.iter().map(|&s| trailheads(s, &grid, false)).sum();
    let gold: usize = starts.iter().map(|&s| trailheads(s, &grid, true)).sum();

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn trailheads(start: (usize, usize), grid: &Vec<Vec<u8>>, count_paths: bool) -> usize {
    let mut seen = HashSet::new();
    let mut tot = 0;
    let mut q = vec![start];

    while let Some(curr @ (y, x)) = q.pop() {
        seen.insert(curr);
        let h = grid[y][x];
        if h == 9 {
            tot += 1;
            continue;
        }

        [
            ((y + 1).min(grid.len() - 1), x),
            (y, (x + 1).min(grid[0].len() - 1)),
            (y.saturating_sub(1), x),
            (y, x.saturating_sub(1)),
        ]
        .iter()
        .filter(|&&(ny, nx)| grid[ny][nx] == h + 1)
        .filter(|next| count_paths || !seen.contains(next))
        .for_each(|&n| q.push(n));
    }
    tot
}
