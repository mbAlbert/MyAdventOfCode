mod part1;
mod part2;

use std::collections::HashMap;

fn main() {
    let input: Vec<_> = include_str!("input.txt").split("\n\n").collect();
    let mut wires: HashMap<_,_> = input[0]
        .lines()
        .map(|line| {
            let wire: Vec<_> = line.split(": ").collect();
            (wire[0], Some(wire[1].parse::<u8>().unwrap()))
        })
        .fold(HashMap::new(), |mut map, (name, value)| {
            map.insert(name, value);
            map
        });

    let operations = input[1]
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" -> ").collect();
            let input: Vec<_> = parts[0].split_ascii_whitespace().collect();
            let in1 = input[0];
            let op = input[1];
            let in2 = input[2];
            let out = parts[1];
            wires.insert(out, None);
            (out, in1, op, in2)
        })
        .fold(HashMap::new(), |mut map, (out, in1, op, in2)| {
            map.insert(out, (in1, op, in2));
            map
        });

        println!("Part 1. {}", part1::result(&mut wires, &operations));
        println!("Part 2. {:?}", part2::result(&operations));
}
                                                       