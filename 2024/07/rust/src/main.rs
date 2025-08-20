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

    let silver: u64 = eqns
        .iter()
        .filter(|(ans, nums)| calibration(ans, nums, false))
        .map(|(ans, _)| ans)
        .sum();

    let gold: u64 = eqns
        .iter()
        .filter(|(ans, nums)| calibration(ans, nums, true))
        .map(|(ans, _)| ans)
        .sum();

    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}

fn concat(x: &u64, y: &u64) -> u64 {
    x * (10u64.pow(y.ilog10() + 1)) + y
}

fn calibration(want: &u64, nums: &[u64], gold: bool) -> bool {
    match nums {
        [] => unreachable!(),
        [x] => want == x,
        [x, y, xs @ ..] => {
            if x > want {
                return false;
            }
            calibration(want, &[&[x + y], xs].concat(), gold) ||
            calibration(want, &[&[x * y], xs].concat(), gold) ||
            (gold && calibration(want, &[&[concat(x, y)], xs].concat(), gold) )
        }
    }
}
