use std::{collections::HashMap, io};

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
    let mut pos = None;
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
                pos = Some((x, y))
            }
            '#' => {
                map.insert((x, y), Cell::Obstacle);
            }
            _ => panic!(),
        });

    let mut visited = HashMap::new();
    let mut pos = pos.unwrap();
    let mut dir = Direction::U;

    loop {
        *visited.entry(pos).or_insert(0 as u8) |= dir.clone() as u8;

        let ahead = dir.shift(pos);
        match map.get(&ahead) {
            Some(Cell::Empty) => pos = ahead,
            Some(Cell::Obstacle) => dir = dir.rotate(),
            None => break,
        };
    }

    let silver: usize = visited.len();
    let gold: usize = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
