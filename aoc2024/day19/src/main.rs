use std::collections::HashMap;

fn is_design_possible<'a>(patterns: &Vec<&'a str>, design: &'a str) -> bool {
    fn is_design_possible_cached<'a>(patterns: &Vec<&'a str>, design: &'a str, cache: &mut HashMap<&'a str, bool>) -> bool {
        if let Some(&found) = cache.get(design) {
            return found;
        }

        for &pattern in patterns {
            if pattern == design {
                return true;
            }

            if design.starts_with(pattern) {
                if is_design_possible_cached(patterns, &design[pattern.len()..], cache) {
                    cache.insert(design, true);
                    return true;
                } else {
                    cache.insert(design, false);
                }
            }
        }

        false
    }

    let mut cache = HashMap::new();
    is_design_possible_cached(patterns, design, &mut cache)
}

fn design_count<'a>(patterns: &Vec<&'a str>, design: &'a str) -> usize {
    fn design_count_cached<'a>(patterns: &Vec<&'a str>, design: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(&count) = cache.get(design) {
            return count;
        }

        let mut count = 0;
        for &pattern in patterns {
            if pattern == design {
                count += 1;
            }

            if design.starts_with(pattern) {
                count += design_count_cached(patterns, &design[pattern.len()..], cache);
            }
        }

        cache.insert(design, count);
        count
    }

    let mut cache = HashMap::new();
    design_count_cached(patterns, design, &mut cache)
}

fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .split("\n\n").collect();

    let patterns: Vec<_> = input[0].split(", ").collect();
    let designs = input[1];

    let result_p1 = designs
        .lines()
        .filter(|&design| is_design_possible(&patterns, design))
        .count();

    println!("Part 1. {}", result_p1);


    let result_p2 = designs
        .lines()
        .map(|design| design_count(&patterns, design))
        .sum::<usize>();

    println!("Part 2. {}", result_p2);
}
