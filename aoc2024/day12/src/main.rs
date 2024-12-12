mod part1;
mod part2;

fn main() {
    let plot_map: Vec<Vec<_>> = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    println!("{:?}", part1::total_price(&plot_map));
    println!("{:?}", part2::total_price(&plot_map));
}
