use std::collections::HashSet;

fn find_start(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'S' {
                return Some((r, c));
            }
        }
    }

    None
}

fn build_path(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    fn dfs(map: &Vec<Vec<char>>, seen: &mut HashSet<(usize, usize)>, path: &mut Vec<(usize, usize)>, pos: (usize, usize)) {
        let y = pos.0;
        let x = pos.1;
        if map[y][x] == 'E' {
            return;
        }

        let dirs = [(-1, 0), (0, 1), (1, 0), (0 , -1)];
        for (dy, dx) in dirs {
            let ny = (y as isize + dy) as usize;
            let nx = (x as isize + dx) as usize;

            if seen.contains(&(ny, nx)) {
                continue;
            }

            if map[ny][nx] != '#' {
                seen.insert((ny, nx));
                path.push((ny, nx));
                dfs(map, seen, path, (ny, nx));
            }
        }
    }

    let mut path = Vec::new();
    let mut seen = HashSet::new();
    let start = find_start(map).unwrap();
    path.push(start);
    seen.insert(start);
    dfs(map, &mut seen, &mut path, start);
    path
}

fn find_cheats(path: &Vec<(usize, usize)>, savings_ps: usize, cheat_ps: usize) -> usize {
    let mut cheats = 0;
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let time1 = i;
            let time2 = j;
            let (y1, x1) = path[i]; 
            let (y2, x2) = path[j];
            let distance =  y1.abs_diff(y2) + x1.abs_diff(x2);
            if distance <= cheat_ps && ((time2 - time1 - distance) >= savings_ps) {
                cheats += 1;
                // println!("From Node {:?} to Node {:?} with savings: {} ps", path[i], path[j], j - i - distance);
            }  
        }
    }

    cheats
}

fn main() {
    let map: Vec<Vec<_>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let path = build_path(&map);

    println!("Part 1. {}", find_cheats(&path, 100, 2));
    println!("Part 2. {}", find_cheats(&path, 100, 20));    
}
