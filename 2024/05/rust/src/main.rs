#![feature(slice_split_once)]

use std::cmp::Ordering::{Greater, Less};
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines().flatten();
    let mut set = HashSet::<(u16, u16)>::new();

    while let Some(s) = lines.next() {
        if let Some((a, b)) = s.split_once('|') {
            set.insert((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            break;
        }
    }

    let mut silver: usize = 0;
    let mut gold: usize = 0;

    for line in lines {
        let mut nums: Vec<u16> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let mid = nums.len() / 2;

        if nums.is_sorted_by(|&a, &b| set.contains(&(a, b))) {
            silver += nums[mid] as usize;
        } else {
            gold += *nums
                .select_nth_unstable_by(mid, |&a, &b| match set.contains(&(a, b)) {
                    true => Less,
                    false => Greater,
                })
                .1 as usize;
        }
    }

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
