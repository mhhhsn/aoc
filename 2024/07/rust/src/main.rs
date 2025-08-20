use std::io;

fn main() -> io::Result<()> {
    let eqns = io::stdin()
        .lines()
        .flatten()
        .collect::<Vec<_>>()
        .iter()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(ans, nums)| {
            (
                ans.parse::<u64>().unwrap(),
                nums.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let silver: u64 = eqns.iter().map(crate::silver).sum();
    let gold: u64 = eqns.iter().map(crate::gold).sum();

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn concat(x: &u64, y: &u64) -> u64 {
    x * (10u64.pow(y.ilog10() + 1)) + y
}

fn calibration(want: u64, x: &u64, y: &u64, rest: &[u64], gold: bool) -> bool {
    if *x > want {
        return false;
    }
    match rest {
        [] => want == x + y || want == x * y || (gold && want == concat(x, y)),
        [z, rest @ ..] => {
            calibration(want, &(x + y), z, rest, gold)
                || calibration(want, &(x * y), z, rest, gold)
                || (gold && calibration(want, &concat(x, y), z, rest, gold))
        }
    }
}

fn silver((want, nums): &(u64, Vec<u64>)) -> u64 {
    match nums.as_slice() {
        [x, y, rest @ ..] => {
            if calibration(*want, x, y, rest, false) {
                *want
            } else {
                0
            }
        }
        _ => 0,
    }
}

fn gold((want, nums): &(u64, Vec<u64>)) -> u64 {
    match nums.as_slice() {
        [x, y, rest @ ..] => {
            if calibration(*want, x, y, rest, true) {
                *want
            } else {
                0
            }
        }
        _ => 0,
    }
}
