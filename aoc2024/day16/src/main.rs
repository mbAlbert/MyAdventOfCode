mod part1;
mod part2;

fn main() {
    let maze: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    println!("Part 1. {}", part1::result(&maze));
    println!("Part 1. {}", part2::result(&maze));
}
