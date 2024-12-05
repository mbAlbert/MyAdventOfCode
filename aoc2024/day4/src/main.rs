fn dfs(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize, 
    word: &Vec<char>,
    word_idx: usize,
) -> usize {
    if x >= grid.len()
        || y >= grid[0].len()
        || grid[x][y] != word[word_idx] {
        return 0 as usize;
    }

    if word_idx == (word.len()-1) {
        return 1;
    }

    let next_x = x as isize + dx;
    let next_y = y as isize + dy;

    if (next_x < 0) || (next_y < 0) {
        return 0 as usize;
    } 

    return dfs(grid, next_x as usize, next_y as usize, dx, dy, word, word_idx + 1);
}

fn word_count_in_grid(grid: &Vec<Vec<char>>, word: &str) -> usize
{
    let mut total_count: usize = 0;
    let word: Vec<char> = word.chars().collect();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == word[0] {
                // All 8 directions
                total_count += dfs(grid, r, c, -1, -1, &word, 0);
                total_count += dfs(grid, r, c, -1, 0, &word, 0);
                total_count += dfs(grid, r, c, -1, 1, &word, 0);
                total_count += dfs(grid, r, c, 0, -1, &word, 0);
                total_count += dfs(grid, r, c, 0, 1, &word, 0);
                total_count += dfs(grid, r, c, 1, -1, &word, 0);
                total_count += dfs(grid, r, c, 1, 0, &word, 0);
                total_count += dfs(grid, r, c, 1, 1, &word, 0);
            }
        }
    }

    total_count
}

fn is_a_x_mas(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let ul = (x-1, y-1);
    let ur = (x-1, y+1);
    let dl = (x+1, y-1);
    let dr = (x+1, y+1);

    if ( ( ( grid[ul.0][ul.1] == 'M' ) && ( grid[dr.0][dr.1] == 'S' ) )
        || ( ( grid[ul.0][ul.1] == 'S' ) && ( grid[dr.0][dr.1] == 'M' ) ) )
        &&
        ( ( ( grid[ur.0][ur.1] == 'M' ) && ( grid[dl.0][dl.1] == 'S' ) )
        || ( ( grid[ur.0][ur.1] == 'S' ) && ( grid[dl.0][dl.1] == 'M' ) ) )
    {
        return true;
    }

    false
}

fn x_mas_count(grid: &Vec<Vec<char>>) -> usize {
    let mut total_count: usize = 0;
    for r in 1..grid.len()-1 {
        for c in 1..grid[0].len()-1 {
            if grid[r][c] == 'A' && is_a_x_mas(grid, r, c) {
                total_count += 1;
            }
        }
    }

    total_count
}

fn main() {
    let grid: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let vec_chars: Vec<char> = line.trim().chars().collect();
            vec_chars
        })
        .collect();

    // Part 1
    let count = word_count_in_grid(&grid, "XMAS".into());
    println!("Part1. {:?}", count);

    // Part 2
    let count = x_mas_count(&grid);
    println!("Part2: {:?}", count);
}
