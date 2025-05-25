pub fn get_starting_position(grid: &Vec<Vec<char>>) -> (isize, isize) {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '^' {
                return (r as isize, c as isize);
            }
        }
    }

    (0, 0)
}

fn get_next_char(grid: &Vec<Vec<char>>, y: isize, x: isize) -> char {
    if y < 0 || y as usize >= grid.len() || x < 0 || x as usize >= grid[0].len()
    {
        return '*';
    }

    grid[y as usize][x as usize]
}

pub fn walk_grid(grid: &mut Vec<Vec<char>>) -> usize {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;
    let (mut y, mut x) = get_starting_position(grid);
    let mut acc = 0;

    loop {
        if grid[y as usize][x as usize] != 'X' {
            grid[y as usize][x as usize] = 'X';
            acc += 1;
        }

        // Check next position
        let (dy, dx) = directions[dir];
        match get_next_char(grid, y + dy, x + dx) {
            '*' => return acc,
            '#' => dir = (dir + 1) % 4,
            _ => {
                y += dy;
                x += dx;
            }  
        }
    }
}