use std::{
    collections::{HashMap, HashSet},
    io,
};

fn main() -> io::Result<()> {
    let mut antennae: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let grid = io::stdin()
        .lines()
        .flatten()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let max = grid.first().unwrap().len() as i32;
    let may = grid.len() as i32;
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c != '.')
                .map(move |(x, &c)| (c, (x as i32, y as i32)))
        })
        .for_each(|(c, p)| antennae.entry(c).or_default().push(p));

    let mut silver = HashSet::new();
    let mut gold = HashSet::new();
    for ps in antennae.values() {
        for (n, &p) in ps.iter().enumerate() {
            for &t in &ps[n+1..] {
                antinodes(p, t, (max, may), false).for_each(|n| {
                    silver.insert(n);
                });
                antinodes(p, t, (max, may), true).for_each(|n| {
                    gold.insert(n);
                });
            }
        }
    }

    let silver = silver.len();
    let gold = gold.len();
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn antinodes(
    a: (i32, i32),
    b: (i32, i32),
    max: (i32, i32),
    gold: bool,
) -> impl Iterator<Item = (i32, i32)> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let in_range = move |(x, y)| (0..max.0).contains(&x) && (0..max.1).contains(&y);
    if !gold {
        vec![(a.0 - dx, a.1 - dy), (b.0 + dx, b.1 + dy)]
    } else {
        (0..)
            .map(|n| (a.0 - n * dx, a.1 - n * dy))
            .take_while(move |&p| in_range(p))
            .chain(
                (0..)
                    .map(|n| (b.0 + n * dx, b.1 + n * dy))
                    .take_while(move |&p| in_range(p)),
            )
            .collect()
    }
    .into_iter()
    .filter(move |&p| in_range(p))
}
