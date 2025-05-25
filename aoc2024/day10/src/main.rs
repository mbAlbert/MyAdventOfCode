mod part1;
mod part2;

fn main() {
    let topographic_map: Vec<Vec<_>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(16).unwrap_or(0xf)).collect()
        })
        .collect();

    println!("Part 1. {}", part1::trailhead_score(&topographic_map));
    println!("Part 2. {}", part2::trailhead_score(&topographic_map));
}
