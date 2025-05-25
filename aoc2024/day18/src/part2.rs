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

fn ram_corrupt_until_n(ram: &mut Vec<Vec<char>>, bytes: &Vec<(usize, usize)>, n: usize) {
    for i in 0..n {
        let x = bytes[i].1;
        let y = bytes[i].0;
        ram[y][x] = '#';
    }
}

fn ram_corrupt_n(ram: &mut Vec<Vec<char>>, bytes: &Vec<(usize, usize)>, n: usize) {
    if n < bytes.len() {
        let x = bytes[n].1;
        let y = bytes[n].0;
        ram[y][x] = '#';
    }
}

pub fn result() -> (usize, usize) {
    let corrupted_bytes: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let pos: Vec<_> = line.split(",").collect();
            (pos[1].parse::<usize>().unwrap(), pos[0].parse::<usize>().unwrap())
        })
        .collect();

    const HEIGHT: usize = 71;
    const WIDTH: usize = 71;
    let mut ram = vec![vec!['.'; WIDTH]; HEIGHT];
    // Start from part 1 bytes
    let mut n = 1024;
    ram_corrupt_until_n(&mut ram, &corrupted_bytes, n);

    while let Some(_) = shortest_path(&ram) {
        n += 1;
        ram_corrupt_n(&mut ram, &corrupted_bytes, n);
    }

    (corrupted_bytes[n].1, corrupted_bytes[n].0) 
}