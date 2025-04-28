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

    let silver: usize = input.iter().map(self::safe).flatten().count();
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");
}

#[derive(Debug, PartialEq)]
enum Dir {
    Asc,
    Desc,
}

fn safe(xs: &Vec<i64>) -> Option<Dir> {
    let mut dirs = xs
        .iter()
        .zip(xs.iter().skip(1))
        .map(|(x, y)| y - x)
        .map(|d| match d {
            -3..=-1 => Some(Dir::Desc),
            1..=3 => Some(Dir::Asc),
            _ => None,
        })
        .collect::<Option<Vec<Dir>>>()?
        .into_iter();

    let head = dirs.next().unwrap_or(Dir::Asc);
    if dirs.all(|d| d == head) {
        Some(head)
    } else {
        None
    }
}
