#![feature(slice_split_once)]

use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines().flatten();
    let mut heap = HashSet::<(u16, u16)>::new();

    while let Some(s) = lines.next() {
        if let Some((a, b)) = s.split_once('|') {
            heap.insert((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            break;
        }
    }

    let mut silver: usize = 0;
    let mut gold: usize = 0;

    for line in lines {
        let mut nums: Vec<u16> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let presort = nums.clone();
        let mid = nums.len() / 2;
        nums.sort_unstable_by(|&a, &b| match heap.contains(&(a, b)) {
            true => Ordering::Less,
            false => Ordering::Greater,
        });

        if nums.iter().eq(presort.iter()) {
            silver += presort[mid] as usize;
        } else {
            gold += nums[mid] as usize;
        }
    }

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
