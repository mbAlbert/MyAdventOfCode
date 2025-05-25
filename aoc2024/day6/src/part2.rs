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

fn walk_grid_obstructed(grid: &Vec<Vec<char>>, y0: isize, x0: isize) -> bool {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;
    let mut y = y0;
    let mut x = x0;
    let mut walked = vec![vec![vec![false; 4]; grid[0].len()]; grid.len()]; 

    loop {
        if walked[y as usize][x as usize][dir] {
            return true;
        }

        walked[y as usize][x as usize][dir] = true;
        let (dy, dx) = directions[dir];

        match get_next_char(grid, y + dy, x + dx) {
            '*' => return false,
            '#' => dir = (dir + 1) % 4,
            _ => {
                y += dy;
                x += dx;
            }  
        }
    }
}

pub fn walk_grid(grid: &mut Vec<Vec<char>>) -> usize {
    let (y, x) = get_starting_position(grid);
    let mut acc = 0;
    
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                grid[r][c] = '#';
                if walk_grid_obstructed(&grid, y, x) {
                    acc += 1;
                }
                grid[r][c] = '.';
            }
        }
    }

    acc
}