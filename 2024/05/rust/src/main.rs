#![feature(slice_split_once)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

fn main() -> io::Result<()> {
    let mut lines = io::stdin().lines().flatten();
    let mut heap = BinaryHeap::<(u16, u16)>::new();

    while let Some(s) = lines.next() {
        if let Some((a, b)) = s.split_once('|') {
            heap.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            break;
        }
    }
    let order = heap.into_sorted_vec();

    let mut silver = 0;
    let mut gold = 0;

    for line in lines {
        let mut nums: Vec<u16> = line.split(',').map(|n| n.parse().unwrap()).collect();
        let presort = nums.clone();
        let mid = nums.len() / 2;

        nums.sort_unstable_by(|&a, &b| match order.binary_search(&(a, b)) {
            Ok(_) => Ordering::Less,
            Err(_) => Ordering::Greater,
        });

        if nums.iter().eq(presort.iter()) {
            silver += presort[mid];
        } else {
            gold += nums[mid];
        }
    }

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
