mod part1;
mod part2;

use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            let link: Vec<_> = line.split('-').collect();
            (link[0], link[1])
        })
        .fold(HashMap::new(), |mut map, (r1, r2)| {
            map.entry(r1).or_insert_with(HashSet::new).insert(r2);
            map.entry(r2).or_insert_with(HashSet::new).insert(r1);
            map
        });

        println!("Part 1. {}", part1::result(&input));
        let p2 = part2::result(&input);
        print!("Part 2. ");
        for item in &p2 {
            print!("{},", item);
        }
        println!("");
}
