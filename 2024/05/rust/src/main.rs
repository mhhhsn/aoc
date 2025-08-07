#![feature(slice_split_once)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let lines = io::stdin().lines().flatten().collect::<Vec<_>>();
    let (rules, updates) = lines.split_once(String::is_empty).unwrap();
    let manual = rules.iter().collect::<Manual>();

    let updates = updates
        .iter()
        .map(|up| {
            up.split(',')
                .map(|p| Page {
                    number: p.parse::<u64>().unwrap(),
                    manual: &manual,
                })
                .collect::<_>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut silver = 0;
    let mut gold = 0;
    for mut up in updates {
        let mid = up.len() / 2;
        if up.is_sorted() {
            silver += up[mid].number;
        } else {
            up.sort_unstable();
            gold += up[mid].number;
        }
    }

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
#[derive(Debug)]
struct Manual {
    order: HashMap<u64, Vec<u64>>,
}

impl<'a> FromIterator<&'a String> for Manual {
    fn from_iter<T: IntoIterator<Item = &'a String>>(rules: T) -> Self {
        let mut order = HashMap::<u64, Vec<u64>>::new();
        for rule in rules {
            let (pre, post) = rule.split_once('|').unwrap();
            order
                .entry(pre.parse().unwrap())
                .or_default()
                .push(post.parse().unwrap());
        }
        Manual { order }
    }
}

#[derive(Clone, Copy)]
struct Page<'a> {
    number: u64,
    manual: &'a Manual,
}

impl PartialEq for Page<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
impl Eq for Page<'_> {}

impl PartialOrd for Page<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.manual.order.get(&self.number)?.contains(&other.number) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Page<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
