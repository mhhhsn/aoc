use std::io::{self, Read};

#[derive(Debug)]
enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

impl From<&Instruction> for u64 {
    fn from(val: &Instruction) -> Self {
        match val {
            Instruction::Mul(x, y) => x * y,
            _ => 0,
        }
    }
}

fn main() -> io::Result<()> {
    let mut program = String::new();
    io::stdin().read_to_string(&mut program)?;

    let instrs = (0..program.len())
        .filter_map(|idx| {
            let next = &program[idx..];
            if next.starts_with("do()") {
                Some(Instruction::Do)
            } else if next.starts_with("don't()") {
                Some(Instruction::Dont)
            } else if next.starts_with("mul(") {
                let mut first = 0;
                let mut curr = 0;

                for ch in next.bytes().skip(4) {
                    match (ch, curr) {
                        ((b'0'..=b'9'), _) => curr = curr * 10 + (ch - b'0') as u64,
                        (_, 0) => return None,
                        (b',', _) => {
                            first = curr;
                            curr = 0;
                        }
                        (b')', _) => {
                            return Some(Instruction::Mul(first, curr));
                        }
                        _ => return None,
                    };
                }
                None
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let silver: u64 = instrs.iter().map(u64::from).sum();
    let (gold, _) = instrs
        .iter()
        .fold((0 as u64, true), |(acc, on), instr| match (instr, on) {
            (Instruction::Do, _) => (acc, true),
            (Instruction::Dont, _) => (acc, false),
            (Instruction::Mul(_, _), true) => (acc + u64::from(instr), true),
            _ => (acc, on),
        });
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
