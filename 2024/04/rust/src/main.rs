use std::io;

fn main() -> io::Result<()> {
    let grid = io::stdin()
        .lines()
        .flatten()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let grid = Grid(grid);

    let silver: usize = silver(&grid);
    let gold: usize = gold(&grid);

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn at(&self, (x, y): (isize, isize)) -> Option<&char> {
        self.0
            .get(usize::try_from(y).ok()?)?
            .get(usize::try_from(x).ok()?)
    }

    fn check<'a>(
        &self,
        (x, y): (isize, isize),
        mut pat: impl Iterator<Item = &'a ((isize, isize), char)>,
    ) -> bool {
        pat.all(|((dx, dy), ch)| self.at((x + dx, y + dy)).is_some_and(|c| c == ch))
    }

    fn pts(&self) -> impl Iterator<Item = (isize, isize)> + use<'_> {
        (0..self.0[0].len()).flat_map(|x| (0..self.0.len()).map(move |y| (x as isize, y as isize)))
    }
}

fn silver(grid: &Grid) -> usize {
    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let xmases: Vec<Vec<((isize, isize), char)>> = dirs
        .iter()
        .map(|(dx, dy)| {
            (0..4)
                .map(move |n| (dx * n, dy * n))
                .zip("XMAS".chars())
                .collect()
        })
        .collect();

    grid.pts()
        .flat_map(|p| xmases.iter().map(move |xmas| grid.check(p, xmas.iter())))
        .filter(|p| *p)
        .count()
}

fn gold(grid: &Grid) -> usize {
    let dirs = vec![(-1, -1), (1, -1), (0, 0), (-1, 1), (1, 1)];

    let xmases: Vec<Vec<((isize, isize), char)>> = vec!["MMASS", "SMASM", "SSAMM", "MSAMS"]
        .iter()
        .map(|x| dirs.clone().into_iter().zip(x.chars()).collect())
        .collect();

    grid.pts()
        .flat_map(|p| xmases.iter().map(move |xmas| grid.check(p, xmas.iter())))
        .filter(|p| *p)
        .count()
}
