use crate::robot::Robot;

pub fn result() -> i32 {
    const WIDTH_TILES: i32 = 101;
    const HEIGHT_TILES: i32 = 103;
    const SECONDS: i32 = 100;
    const MID_POINT: (i32, i32) = ( ((WIDTH_TILES - 1) / 2), ((HEIGHT_TILES - 1) / 2) ); 

    include_str!("input.txt")
        .lines()
        .map(|line| { 
            let robot = line.parse::<Robot>().unwrap();
            let x = (robot.position.0 + (robot.velocity.0 * SECONDS)) % WIDTH_TILES;
            let x = if x < 0 { x +  WIDTH_TILES } else { x };
            let y = (robot.position.1 + (robot.velocity.1 * SECONDS)) % HEIGHT_TILES;
            let y = if y < 0 { y +  HEIGHT_TILES } else { y };

            (x, y)
        })
        .filter(|pos| (pos.0 != MID_POINT.0) && (pos.1 != MID_POINT.1))
        .fold(vec![0; 4], |mut quadrants, pos| {
            let binary_index = ( (pos.0 - MID_POINT.0) > 0, (pos.1 - MID_POINT.1) > 0 );
            let index = binary_index.0 as usize + 2 * binary_index.1 as usize;
            quadrants[index] += 1;

            quadrants
        })
        .into_iter()
        .product::<i32>()
}