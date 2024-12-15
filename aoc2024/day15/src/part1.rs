fn run_move(map: &mut Vec<Vec<char>>, obj: char, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let y = pos.0 as usize;
    let ny = (pos.0 + dir.0) as usize; 
    let x = pos.1 as usize;
    let nx = (pos.1 + dir.1) as usize;

    if map[ny][nx] == '#' {
        return false;
    }

    if map[ny][nx] == 'O' {
        run_move(map, 'O', (ny as i32, nx as i32), dir);
    }

    if map[ny][nx] == '.' {
        map[y][x] = '.';
        map[ny][nx] = obj;
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
            if map[r][c] == 'O' {
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
        .map(|line| line.chars().collect())
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
            if run_move(&mut map, '@', robot_pos, mov) {
                robot_pos.0 += mov.0;
                robot_pos.1 += mov.1;
            }
        }
    }


    box_gps_score(&map)
}