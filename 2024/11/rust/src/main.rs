use std::{collections::HashMap, io};

fn blink(k @ (s, n): (u64, u64), cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    if let Some(v) = cache.get(&k) {
        return *v;
    }

    // cache this
    let v = {
        if s == 0 {
            return blink((1, n - 1), cache);
        }
        let digits = s.ilog10() + 1;
        let mid = 10u64.pow(digits / 2);
        if digits % 2 == 0 {
            blink((s / mid, n - 1), cache) + blink((s % mid, n - 1), cache)
        } else {
            blink((s * 2024, n - 1), cache)
        }
    };

    *cache.entry(k).or_insert(v)
}

fn main() -> io::Result<()> {
    let line = io::stdin().lines().flatten().next().unwrap();
    let stones = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let silver: u64 = stones.iter().map(|s| blink((*s, 25), &mut cache)).sum();
    let gold: u64 = stones.iter().map(|s| blink((*s, 75), &mut cache)).sum();

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
