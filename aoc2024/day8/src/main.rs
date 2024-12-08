mod part1;
mod part2;

fn main() {
    let map: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    println!("Part 1. {:?}", part1::antinode_count(&map));
    println!("Part 2. {:?}", part2::antinode_count(&map));
}
