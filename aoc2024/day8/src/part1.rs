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

fn get_antenna_pair_antinodes(antenna1: (isize, isize), antenna2: (isize, isize)) -> ((isize, isize), (isize, isize)) {
    let y1 = antenna1.0;
    let y2 = antenna2.0;
    let x1 = antenna1.1;
    let x2 = antenna2.1;
    let dy = (y1-y2).abs();
    let dx = (x1-x2).abs();
    let antenna_layout = (y1 < y2, x1 < x2);

    match antenna_layout {
        (false, false) => ((y1 + dy, x1 + dx),(y2 - dy, x2 - dx)),
        (false, true) => ((y1 + dy, x1 - dx),(y2 - dy, x2 + dx)),
        (true, false) => ((y1 - dy, x1 + dx),(y2 + dy, x2 - dx)),
        (true, true) => ((y1 - dy, x1 - dx),(y2 + dy, x2 + dx)),
    }
}

fn get_antinode_positions(antenna_positions: &HashMap<char, Vec<(isize, isize)>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut antinode_positions = HashMap::new();

    for (key, value) in antenna_positions {
        for i in 0..value.len()-1 {
            for j in i+1..value.len() {
                let antinode_pair = get_antenna_pair_antinodes(value[i], value[j]);
                antinode_positions.entry(*key).or_insert_with(Vec::new).push(antinode_pair.0);
                antinode_positions.entry(*key).or_insert_with(Vec::new).push(antinode_pair.1);
            }
        }
    }

    antinode_positions
}

fn get_antinode_count(antinode_positions: &HashMap<char, Vec<(isize, isize)>>, map_size: (isize, isize)) -> usize {
    let mut unique_positions = HashSet::new();

    for positions in antinode_positions.values() {
        for pos in positions {
            if pos.0 >= 0 && pos.0 < map_size.0 && pos.1 >= 0 && pos.1 < map_size.1 {
                unique_positions.insert(*pos);
            }
        }
    }

    unique_positions.len()
}

pub fn antinode_count(map: &Vec<Vec<char>>) -> usize {
    get_antinode_count(&get_antinode_positions(&get_antenna_positions(map)), (map.len() as isize, map[0].len() as isize))
}