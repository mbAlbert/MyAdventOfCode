mod part1;
mod part2;

fn main() {
    let stones: Vec<_> = include_str!("input.txt")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    println!("Part1. {:?}", part1::blink(&stones, 25));
    println!("Part2. {:?}", part2::blink(&stones, 75));
}
