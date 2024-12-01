use std::io;

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        let _line = line?;
    }

    let silver: u64 = 0;
    let gold: u64 = 0;
    println!("silver: {silver}");
    println!("gold: {gold}");

    return Ok(());
}
