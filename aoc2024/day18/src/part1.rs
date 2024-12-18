fn ram_corrupt(ram: &mut Vec<Vec<char>>, bytes: &Vec<(usize, usize)>, ns: usize) {
    for i in 0..ns {
        let x = bytes[i].1;
        let y = bytes[i].0;
        ram[y][x] = '#';
    }
}

fn shortest_path(ram: &Vec<Vec<char>>) -> Option<usize> {
    let dirs: [(isize, isize); 4] = [(-1,0), (0,1), (1,0), (0,-1)];

    let mut queue = std::collections::VecDeque::new();
    let mut seen = vec![vec![false; ram[0].len()]; ram.len()];
    queue.push_front(((0,0), 0));

    while let Some(((y, x), steps)) = queue.pop_back() {
        if seen[y][x] {
            continue;
        }
        seen[y][x] = true;

        if (y,x) == (ram.len() - 1, ram[0].len() - 1) {
            return Some(steps);
        }

        for (dy, dx) in dirs {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny < 0 || ny >= ram.len() as isize || nx < 0 || nx >= ram[0].len() as isize {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if seen[ny][nx] {
                continue;
            }

            if ram[ny][nx] == '#' {
                continue;
            }

            queue.push_front(((ny, nx), steps + 1));
        }
    }

    None
}

pub fn result() -> usize {
    let corrupted_bytes: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let pos: Vec<_> = line.split(",").collect();
            (pos[1].parse::<usize>().unwrap(), pos[0].parse::<usize>().unwrap())
        })
        .collect();

    const HEIGHT: usize = 71;
    const WIDTH: usize = 71;
    const TIME_NS: usize = 1024;
    let mut ram = vec![vec!['.'; WIDTH]; HEIGHT];
    ram_corrupt(&mut ram, &corrupted_bytes, TIME_NS);
    shortest_path(&ram).unwrap()
}