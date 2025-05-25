use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::usize;
use bimap::BiMap;
use itertools::Itertools;

fn bfs(keypad: &BiMap<char, (isize, isize)>, s_key: char, e_key: char) -> Vec<String> {
    let dirs = [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')];
    let &s_pos = keypad.get_by_left(&s_key).unwrap();
    let &e_pos = keypad.get_by_left(&e_key).unwrap();

    // Queue -> (Current Position, Path String)
    let mut queue = VecDeque::new();
    queue.push_back((s_pos, "".to_string(), HashSet::from([s_pos])));
    
    let mut paths = Vec::new();
    let mut min_length = None;

    while let Some((c_pos, path, visited)) = queue.pop_front() {
        if c_pos == e_pos {
            if let Some(min) = min_length {
                if path.len() == min {
                    paths.push(path)
                }
            } else {
                min_length = Some(path.len());
                paths.push(path)
            }
            continue;
        }

        for &(dy, dx, dir) in &dirs {
            let n_pos = (c_pos.0 + dy, c_pos.1 + dx);
           
            if let Some(&n_key) = keypad.get_by_right(&n_pos) {
                if !visited.contains(&n_pos) || n_key == e_key {
                    let mut n_visited = visited.clone();
                    n_visited.insert(n_pos);
                    queue.push_back((n_pos, format!("{}{}", path, dir), n_visited));
                }
            }
        }
    }

    paths
}

fn numeric_keypad_paths() -> HashMap<(char, char), Vec<String>> {
    // Layout
    // | 7 | 8 | 9 |
    // | 4 | 5 | 6 |
    // | 1 | 2 | 3 |
    //     | 0 | A |

    let mut keypad: BiMap<char, (isize, isize)> = BiMap::new();
    keypad.insert('7', (0, 0));
    keypad.insert('8', (0, 1));
    keypad.insert('9', (0, 2));
    keypad.insert('4', (1, 0));
    keypad.insert('5', (1, 1));
    keypad.insert('6', (1, 2));
    keypad.insert('1', (2, 0));
    keypad.insert('2', (2, 1));
    keypad.insert('3', (2, 2));
    keypad.insert('0', (3, 1));
    keypad.insert('A', (3, 2));

    let mut paths = HashMap::new();
    for &s_key in keypad.left_values() {
        for &e_key in keypad.left_values() {
            paths.insert((s_key, e_key), bfs(&keypad, s_key, e_key));
        }
    }

    paths
}

fn directional_keypad_paths() -> HashMap<(char, char), Vec<String>> {
    // Layout
    //     | ^ | A |
    // | < | v | > |
    
    let mut keypad: BiMap<char, (isize, isize)> = BiMap::new();
    keypad.insert('^', (0, 1));
    keypad.insert('A', (0, 2));
    keypad.insert('<', (1, 0));
    keypad.insert('v', (1, 1));
    keypad.insert('>', (1, 2));

    let mut paths = HashMap::new();
    for &s_key in keypad.left_values() {
        for &e_key in keypad.left_values() {
            paths.insert((s_key, e_key), bfs(&keypad, s_key, e_key));
        }
    }

    paths
}

fn shortest(code: String, depth: usize) -> usize {
    fn shortest_directional(directional: &HashMap<(char, char), Vec<String>>, code: String, depth: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    
        if let Some(&v) = cache.get(&(code.clone(), depth)) {
            return v;
        }
        
        let pairs: Vec<(char, char)> = (format!("A{}", code))
            .chars()
            .tuple_windows()
            .collect();
    
        let mut acc = 0;
        for pair in pairs {
            let paths = directional.get(&pair).unwrap();
            if depth == 0 { 
                acc += paths[0].len() + 1;
                continue;
            }
    
            let mut min = usize::MAX;
            for path in paths {
                let mut path = path.clone();
                path.push('A');
                let v= shortest_directional(directional, path, depth - 1, cache);
                if v < min {
                    min = v;
                }
            }
    
            acc += min;
        }
    
        cache.insert((code.clone(), depth), acc);
        acc
    }

    let numerical = numeric_keypad_paths(); 
    let directional = directional_keypad_paths();
    let mut cache = HashMap::new();

    let pairs: Vec<(char, char)> = (format!("A{}", code))
        .chars()
        .tuple_windows()
        .collect();

    let mut acc = 0;
    for pair in pairs {
        let paths = numerical.get(&pair).unwrap();

        let mut min = usize::MAX;
        for path in paths {
            let mut path = path.clone();
            path.push('A');
            let v= shortest_directional(&directional, path, depth - 1, &mut cache);
            if v < min {
                min = v;
            }
        }

        acc += min;
    }

    acc
}

fn main() {
    let p1 = include_str!("input.txt")
        .lines()
        .map(|code| {
            let shortest = shortest(code.to_string(), 2);
            let value = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
            shortest * value
        })
        .sum::<usize>();

    println!("Part 1. {}", p1);

    let p2 = include_str!("input.txt")
        .lines()
        .map(|code| {
            let shortest = shortest(code.to_string(), 25);
            let value = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
            shortest * value
        })
        .sum::<usize>();

    println!("Part 2. {}", p2);
}
