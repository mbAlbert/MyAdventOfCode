fn can_box_move_vertical(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: i32) -> bool {
    // Current box positions
    let l_y;
    let l_x;
    let r_y;
    let r_x;
    if map[pos.0 as usize][pos.1 as usize] == '[' {
        l_y = pos.0 as usize;
        l_x = pos.1 as usize;
        r_y = l_y;
        r_x = l_x + 1;
    } else {
        r_y = pos.0 as usize;
        r_x = pos.1 as usize;
        l_y = r_y;
        l_x = r_x - 1;
    }

    // Next box positions
    let l_ny = (l_y as i32 + dir) as usize;
    let r_ny = (r_y as i32 + dir) as usize;
    let l_nx = l_x;
    let r_nx= r_x;

    if map[l_ny][l_nx] == '#' || map[r_ny][r_nx] == '#' {
        return false;
    }

    let mut can_left_box_move = true;
    if map[l_ny][l_nx] == '[' || map[l_ny][l_nx] == ']' {
        can_left_box_move = can_box_move_vertical(map, (l_ny as i32, l_nx as i32), dir);
    }

    let mut can_right_box_move = true;
    if map[r_ny][r_nx] == '[' || map[r_ny][r_nx] == ']' {
        can_right_box_move = can_box_move_vertical(map, (r_ny as i32, r_nx as i32), dir);
    }

    can_right_box_move && can_left_box_move
}

fn move_box_vertical(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: i32) {
    // Current box positions
    let l_y;
    let l_x;
    let r_y;
    let r_x;
    if map[pos.0 as usize][pos.1 as usize] == '[' {
        l_y = pos.0 as usize;
        l_x = pos.1 as usize;
        r_y = l_y;
        r_x = l_x + 1;
    } else {
        r_y = pos.0 as usize;
        r_x = pos.1 as usize;
        l_y = r_y;
        l_x = r_x - 1;
    }

    // Next box positions
    let l_ny = (l_y as i32 + dir) as usize;
    let r_ny = (r_y as i32 + dir) as usize;
    let l_nx = l_x;
    let r_nx= r_x;

    if map[l_ny][l_nx] == '#' || map[r_ny][r_nx] == '#' {
        return;
    }

    if map[l_ny][l_nx] == '[' || map[l_ny][l_nx] == ']' {
        move_box_vertical(map, (l_ny as i32, l_nx as i32), dir);
    }

    if map[r_ny][r_nx] == '[' || map[r_ny][r_nx] == ']' {
        move_box_vertical(map, (r_ny as i32, r_nx as i32), dir);
    }

    if map[l_ny][l_nx] == '.' && map[r_ny][r_nx] == '.' {
        map[l_y][l_x] = '.';
        map[r_y][r_x] = '.';
        map[l_ny][l_nx] = '[';
        map[r_ny][r_nx] = ']';
    }
}

fn move_box_horizontal(map: &mut Vec<Vec<char>>, obj: char, pos: (i32, i32), dir: i32) {
    let y = pos.0 as usize;
    let ny = y; 
    let x = pos.1 as usize;
    let nx = (pos.1 + dir) as usize;

    if map[ny][nx] == '#' {
        return;
    }

    if map[ny][nx] == '[' || map[ny][nx] == ']' {
        move_box_horizontal(map, map[ny][nx], (ny as i32, nx as i32), dir);
    }

    if map[ny][nx] == '.' {
        map[y][x] = '.';
        map[ny][nx] = obj;
    }
}

fn move_box(map: &mut Vec<Vec<char>>, obj: char, pos: (i32, i32), dir: (i32, i32)) {
    if dir.0 == 0 {
        move_box_horizontal(map, obj, pos, dir.1);
    } else {
        if can_box_move_vertical(map, pos, dir.0) {
            move_box_vertical(map, pos, dir.0);
        }
    }
}

fn move_robot(map: &mut Vec<Vec<char>>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let y = pos.0 as usize;
    let ny = (pos.0 + dir.0) as usize; 
    let x = pos.1 as usize;
    let nx = (pos.1 + dir.1) as usize;

    if map[ny][nx] == '#' {
        return false;
    }

    if map[ny][nx] == '[' || map[ny][nx] == ']' {
        move_box(map, map[ny][nx], (ny as i32, nx as i32), dir);
    }

    if map[ny][nx] == '.' {
        map[y][x] = '.';
        map[ny][nx] = '@';
        return true;
    }

    false
}

fn find_robot(map: &Vec<Vec<char>>) -> Option<(i32, i32)> {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '@' {
                return Some((r as i32, c as i32));
            }
        }
    }

    None
}

fn box_gps_score(map: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '[' {
                score += r as u32 * 100 + c as u32;
            }
        }
    }

    score
}

pub fn result() -> u32 {
    let input: Vec<_> = include_str!("input.txt").split("\n\n").collect();
    let mut map: Vec<Vec<_>> = input[0]
        .lines()
        .map(|line| {
            line
                .chars()
                .fold(Vec::new(), |mut acc, c| {
                    let pair = match c {
                        '#' => Some(('#', '#')),
                        'O' => Some(('[', ']')),
                        '.' => Some(('.', '.')),
                        '@' => Some(('@', '.')),
                        _ => None,
                    };

                    if let Some((c1, c2)) = pair {
                        acc.push(c1);
                        acc.push(c2);
                    }

                    acc
                }) 
        })
        .collect();

    let movements: Vec<_> = input[1]
        .trim()
        .chars()
        .map(|c| {
            match c {
                '^' => Some((-1,  0)),
                'v' => Some(( 1,  0)),
                '>' => Some(( 0,  1)),
                '<' => Some(( 0, -1)),
                 _  => None,
            }
        })
        .flatten()
        .collect();

    if let Some(mut robot_pos) = find_robot(&map) {
        for mov in movements {
            if move_robot(&mut map, robot_pos, mov) {
                robot_pos.0 += mov.0;
                robot_pos.1 += mov.1;
            }
        }
    }

    box_gps_score(&map)
}