use std::collections::{HashMap, HashSet};

fn is_update_correct(priorities: &HashMap<u64, HashSet<u64>>, update: &Vec<u64>) -> bool {
    for (i, &num) in update.iter().enumerate() {
        if let Some(priority) = priorities.get(&num) {
            if update[i + 1..].iter().any(|&n| priority.contains(&n)) {
                return false;
            }
        }
    }

    true
}

fn sort_update(priorities: &HashMap<u64, HashSet<u64>>, update: &mut Vec<u64>) {
    for i in 0..update.len() {
        for j in 0..(update.len() - i - 1) {
            if let Some(priority) = priorities.get(&update[j]) {
                if priority.contains(&update[j+1]) {
                    update.swap(j, j + 1);
                }
            }
        }
    }    
}

fn main() {
    let input: Vec<_> = include_str!("input.txt").split("\n\n").collect();

    let priorities_map = input[0]
        .lines()
        .map(|line| {
            let mut xy = line.trim().split("|");
            let x = xy.next().unwrap().parse::<u64>().unwrap();
            let y = xy.next().unwrap().parse::<u64>().unwrap();
            (y, x)
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert_with(HashSet::new).insert(value);
            acc
        });

    let result_p1: u64 = input[1]
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .filter(|update| {
            is_update_correct(&priorities_map, update)
        })
        .map(|update| {
            update[update.len() / 2]
        })
        .sum();

    println!("Part 1. {:?}", result_p1);

    let result_p2: u64 = input[1]
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect()
        })
        .filter(|update| {
            !is_update_correct(&priorities_map, update)
        })
        .map(|mut update| {
            sort_update(&priorities_map, &mut update);
            update[update.len() / 2]
        })
        .sum();

        println!("Part 2. {:?}", result_p2);

}
