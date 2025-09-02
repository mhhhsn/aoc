#![feature(iter_array_chunks)]

use std::io;

fn main() -> io::Result<()> {
    let disk_map = io::stdin()
        .lines()
        .flatten()
        .next()
        .unwrap()
        .chars()
        .map(|c| c as u8 - b'0')
        .collect();

    let silver: usize = silver(&disk_map);
    let gold: usize = gold(&disk_map);
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn silver(input: &Vec<u8>) -> usize {
    let mut blocks = input
        .iter()
        .chain([0, 0].iter()) // lol
        .array_chunks()
        .enumerate()
        .map(|(n, [&f, &s])| [[Some(n)].repeat(f as usize), [None].repeat(s as usize)])
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    let mut l = 0;
    let mut r = blocks.len() - 1;
    loop {
        while blocks[r].is_none() {
            r -= 1;
        }
        while blocks[l].is_some() {
            l += 1;
        }
        if l >= r {
            break;
        }
        let [left, right] = blocks.get_disjoint_mut([l, r]).unwrap();
        left.replace(right.take().unwrap());
    }

    blocks
        .iter()
        .enumerate()
        .map(|(n, id)| n * id.unwrap_or_default())
        .sum()
}

fn gold(input: &Vec<u8>) -> usize {
    let mut blocks = input
        .iter()
        .chain([0, 0].iter()) // lol
        .array_chunks()
        .enumerate()
        .map(|(n, [&f, &s])| [(Some(n), f as usize), (None, s as usize)])
        .flatten()
        .collect::<Vec<_>>();

    let mut r = blocks.len();
    while r > 0 {
        r -= 1;
        let (Some(_), f) = blocks[r] else {
            continue;
        };

        // find some empty space and maybe split
        let Some(l) = blocks.iter().position(|b| b.0.is_none() && b.1 >= f) else {
            continue;
        };
        if l > r {
            continue;
        };

        let (_, s) = &mut blocks[l];

        let diff = *s - f;
        if diff > 0 {
            *s = f;
            blocks.insert(l + 1, (None, diff));
            r += 1;
        }
        blocks.swap(l, r);
    }

    blocks
        .iter()
        .map(|&(id, n)| [id].repeat(n))
        .flatten()
        .enumerate()
        .map(|(n, id)| n * id.unwrap_or_default())
        .sum()
}
