mod claw_machine;
mod part1;
mod part2;

use claw_machine::ClawMachine;

fn main() {
    let machines: Vec<ClawMachine> = include_str!("input.txt")
        .split("\n\n")
        .map(|machine| machine.parse::<ClawMachine>().unwrap())
        .collect();

    println!("{:?}", part1::result(&machines));
    println!("{:?}", part2::result(&machines));
}
