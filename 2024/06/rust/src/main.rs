use std::collections::HashSet;
use std::{collections::HashMap, io};

#[derive(Clone)]
enum Cell {
    Empty,
    Obstacle,
}

#[derive(Clone)]
enum Direction {
    L = 0b1,
    R = 0b10,
    U = 0b100,
    D = 0b1000,
}

impl Direction {
    fn shift(&self, point: (isize, isize)) -> (isize, isize) {
        let (x, y) = point;
        match self {
            Direction::L => (x - 1, y),
            Direction::R => (x + 1, y),
            Direction::U => (x, y - 1),
            Direction::D => (x, y + 1),
        }
    }

    fn rotate(&self) -> Self {
        use Direction::*;
        match self {
            L => U,
            R => D,
            U => R,
            D => L,
        }
    }
}

fn main() -> io::Result<()> {
    let mut map = HashMap::new();
    let mut start = None;
    io::stdin()
        .lines()
        .flatten()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| ([x as isize, y as isize], ch))
                .collect::<Vec<_>>()
        })
        .for_each(|([x, y], ch)| match ch {
            '.' => {
                map.insert((x, y), Cell::Empty);
            }
            '^' => {
                map.insert((x, y), Cell::Empty);
                start = Some((x, y))
            }
            '#' => {
                map.insert((x, y), Cell::Obstacle);
            }
            _ => panic!(),
        });
    let start = start.unwrap();

    let visited = path_len(&map, start.clone()).unwrap();
    let silver: usize = visited.len();
    let gold: usize = visited
        .iter()
        .filter(|&p| {
            let mut map = map.clone();
            map.insert(*p, Cell::Obstacle);
            path_len(&map, start.clone()).is_none()
        })
        .count();
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn path_len(
    map: &HashMap<(isize, isize), Cell>,
    mut pos: (isize, isize),
) -> Option<HashSet<(isize, isize)>> {
    let mut visited = HashMap::new();
    let mut dir = Direction::U;

    loop {
        let before = visited.entry(pos).or_insert(0 as u8);
        if *before & dir.clone() as u8 > 0 {
            return None;
        }
        *before |= dir.clone() as u8;

        let ahead = dir.shift(pos);
        match map.get(&ahead) {
            Some(Cell::Empty) => pos = ahead,
            Some(Cell::Obstacle) => dir = dir.rotate(),
            None => return Some(visited.into_keys().collect()),
        };
    }
}
