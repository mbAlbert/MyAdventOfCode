use std::collections::{HashMap, HashSet};

fn get_antenna_positions(map: &Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antenna_positions = HashMap::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c].is_ascii_alphanumeric() {
                antenna_positions.entry(map[r][c]).or_insert_with(Vec::new).push((r as isize, c as isize));
            }
        }
    }

    antenna_positions
}

fn get_antenna_pair_antinodes(antenna1: (isize, isize), antenna2: (isize, isize), map_size: (isize, isize)) -> Vec<(isize, isize)> {
    let y1 = antenna1.0;
    let y2 = antenna2.0;
    let x1 = antenna1.1;
    let x2 = antenna2.1;
    let dy = (y1-y2).abs();
    let dx = (x1-x2).abs();
    let antenna_layout = (y1 < y2, x1 < x2);
    
    let (dy1, dy2, dx1, dx2) = match antenna_layout {
        (false, false) => (dy, -dy, dx, -dx),
        (false, true) => (dy, -dy, -dx, dx),
        (true, false) => (-dy, dy, dx, -dx),
        (true, true) =>  (-dy, dy, -dx, dx),
    };
    
    let mut antinodes = Vec::new();
    // The antennas themselves are now antinodes so start adding them:
    antinodes.push(antenna1);
    antinodes.push(antenna2);

    // Antinodes of antenna 1
    let mut antinode: (isize, isize) = (y1 + dy1, x1 + dx1);
    while antinode.0 >= 0 && antinode.0 < map_size.0 && antinode.1 >= 0 && antinode.1 < map_size.1 {
        antinodes.push(antinode);
        antinode.0 += dy1;
        antinode.1 += dx1;
    }

    // Antinodes of antenna 2
    let mut antinode: (isize, isize) = (y2 + dy2, x2 + dx2);
    while antinode.0 >= 0 && antinode.0 < map_size.0 && antinode.1 >= 0 && antinode.1 < map_size.1 {
        antinodes.push(antinode);
        antinode.0 += dy2;
        antinode.1 += dx2;
    }

    antinodes
}

fn get_antinode_positions(antenna_positions: &HashMap<char, Vec<(isize, isize)>>, map_size: (isize, isize)) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antinode_positions = HashMap::new();

    for (key, value) in antenna_positions {
        for i in 0..value.len()-1 {
            for j in i+1..value.len() {
                let antinodes = get_antenna_pair_antinodes(value[i], value[j], map_size);
                for antinode in antinodes {
                    antinode_positions.entry(*key).or_insert_with(Vec::new).push(antinode);
                }
            }
        }
    }

    antinode_positions
}

fn get_antinode_count(antinode_positions: &HashMap<char, Vec<(isize, isize)>>) -> usize {
    let mut unique_positions = HashSet::new();

    for positions in antinode_positions.values() {
        for pos in positions {
            unique_positions.insert(*pos);
        }
    }

    unique_positions.len()
}

pub fn antinode_count(map: &Vec<Vec<char>>) -> usize {
    let map_size = (map.len() as isize, map[0].len() as isize);
    let antinode_positions = get_antinode_positions(&get_antenna_positions(map), map_size);
    get_antinode_count(&antinode_positions)
}