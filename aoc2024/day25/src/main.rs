#[derive(Debug)]
enum Schematic {
    Lock,
    Key
}

fn main() {
    let schematics: Vec<_> = include_str!("input.txt")
        .split("\n\n")
        .map(|schematic| {
            let count_vec = schematic
                .lines()
                .fold(vec![-1; 5], |mut vec, line| {
                    for (i, c) in line.chars().into_iter().enumerate() {
                        if c == '#' {
                            vec[i] += 1;
                        }
                    }

                    vec
                });
            let schematic_type = if schematic.chars().next().unwrap() == '#' {
                Schematic::Lock
            } else {
                Schematic::Key
            };

            (schematic_type, count_vec)
        })
        .collect();

    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for (s_type, s_count) in schematics {
        if let Schematic::Lock = s_type {
            locks.push(s_count);
        } else {
            keys.push(s_count);
        }
    }

    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            let mut fits = true;
            for i in 0..keys[0].len() {
                if lock[i] + key[i] > 5 {
                    fits = false;
                }
            }
            if fits {
                count += 1; 
            }
        }
    }

    println!("Solution. {}", count);
}
