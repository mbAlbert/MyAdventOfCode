use std::fs::File;
use std::io::Write;

use crate::robot::Robot;

const WIDTH_TILES: i32 = 101;
const HEIGHT_TILES: i32 = 103;

fn capture_next_second(robots: &mut Vec<Robot>, grid: &mut Vec<Vec<bool>>) {
    for robot in robots {
        // Erease previous robot position in grid
        grid[robot.position.1 as usize][robot.position.0 as usize] = false;

        // Find next robot position
        let x = (robot.position.0 + robot.velocity.0) % WIDTH_TILES;
        let x = if x < 0 { x +  WIDTH_TILES } else { x };
        let y = (robot.position.1 + robot.velocity.1) % HEIGHT_TILES;
        let y = if y < 0 { y +  HEIGHT_TILES } else { y };

        robot.position = (x, y);

        // Write new position in grid
        grid[robot.position.1 as usize][robot.position.0 as usize] = true;
    }
}

fn found_dense_spot_in_sliding_window(grid: &Vec<Vec<bool>>, window_size: usize, threshold: u32) -> bool {
    // Run grid
    for gr in 0..grid.len() - window_size - 1 {
        for gc in 0..grid[0].len() - window_size - 1 {
            let mut density = 0;
            // Run window
            for wr in 0..window_size {
                for wc in 0..window_size {
                    if grid[gr + wr][gc + wc] {
                        density += 1;
                    }
                }
            }

            if density >= threshold {
                return true;
            }
        }
    }

    false
}

fn print_grid_to_file(grid: &Vec<Vec<bool>>) {
    let mut file = File::create("src/easter_egg.txt").unwrap();

    for line in grid.iter() {
        let line_str: String = line.iter().map(|&b| if b { '1' } else { '.' }).collect();
        writeln!(file, "{}", line_str).unwrap(); 
    }
}

pub fn result() -> u32 {
    let mut robots: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Robot>().unwrap())
        .collect();

    const DENSITY_THRESHOLD: u32 = 9;
    const WINDOW_SIZE: usize = 3;
    let mut seconds = 0;
    let mut grid = vec![vec![false; WIDTH_TILES as usize]; HEIGHT_TILES as usize];
    loop {
        seconds += 1;
        capture_next_second(&mut robots, &mut grid);

        if found_dense_spot_in_sliding_window(&grid, WINDOW_SIZE, DENSITY_THRESHOLD) {
            break;
        }     
    }

    print_grid_to_file(&grid);

    seconds
}