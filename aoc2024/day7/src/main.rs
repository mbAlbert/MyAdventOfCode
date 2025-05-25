use std::collections::HashSet;

fn generate_equation_results_p1(numbers: &Vec<i64>) -> HashSet<i64> {
    let mut results = HashSet::new();

    let mut current_values = vec![numbers[0]];
    for &num in &numbers[1..] {
        let mut next_values = Vec::new();
        for val in current_values {
            next_values.push(val * num);
            next_values.push(val + num);
        }

        current_values = next_values;
    } 

    results.extend(current_values);
    results
}

fn concat_operation(a: i64, b: i64) -> i64 {
    let concat = format!("{}{}", a, b);
    concat.parse::<i64>().unwrap()
}

fn generate_equation_results_p2(numbers: &Vec<i64>) -> HashSet<i64> {
    let mut results = HashSet::new();

    let mut current_values = vec![numbers[0]];
    for &num in &numbers[1..] {
        let mut next_values = Vec::new();
        for val in current_values {
            next_values.push(val * num);
            next_values.push(val + num);
            next_values.push(concat_operation(val, num));
        }

        current_values = next_values;
    } 

    results.extend(current_values);
    results
}

fn main() {
    let result_p1: i64 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let input: Vec<_> = line.split(": ").collect();
            let target = input[0].parse::<i64>().unwrap();
            let numbers = input[1]
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            (target, numbers)
        })
        .map(|equation| {
            (equation.0, generate_equation_results_p1(&equation.1))
        })
        .filter(|equation| {
            equation.1.contains(&equation.0)
        })
        .map(|valid_equation| valid_equation.0)
        .sum();

    println!("Part 1. {:?}", result_p1);

    let result_p2: i64 = include_str!("input.txt")
        .lines()
        .map(|line| {
            let input: Vec<_> = line.split(": ").collect();
            let target = input[0].parse::<i64>().unwrap();
            let numbers = input[1]
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            (target, numbers)
        })
        .map(|equation| {
            (equation.0, generate_equation_results_p2(&equation.1))
        })
        .filter(|equation| {
            equation.1.contains(&equation.0)
        })
        .map(|valid_equation| valid_equation.0)
        .sum();

    println!("Part 2. {:?}", result_p2);
}
