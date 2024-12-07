mod part1;
mod part2;

fn main() {
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut grid_p1 = grid.clone();
    let result_p1 = part1::walk_grid(&mut grid_p1);
    println!("{:?}", result_p1);


    let mut grid_p2 = grid.clone();
    let result_p2 = part2::walk_grid(&mut grid_p2);
    println!("{:?}", result_p2);
}