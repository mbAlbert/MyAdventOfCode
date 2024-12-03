use regex::Regex;

fn main() {
    
    // Part 1
    let program = include_str!("input.txt");
    let re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let result_p1: i64 = re.captures_iter(program)
        .map(|c| {
            let ( Some(first), Some(second) ) = ( c[1].parse::<i64>().ok(), c[2].parse::<i64>().ok() ) else {
                return 0 as i64;
            };

            first * second
        })
        .sum();

    println!("Part1. Result: {result_p1} ");

    // Part 2
    let program = format!("do(){}don't()", include_str!("input.txt"));
    let block_re = Regex::new(r"(?s)(do\(\)).*?(don't\(\))").unwrap();
    let mul_re = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();

    let result_p2: i64 = block_re.captures_iter(&program)
        .map(|cblock| {
            mul_re.captures_iter(cblock.get(0).unwrap().as_str())
                .map(|cmul| {
                    let ( Some(first), Some(second) ) = ( cmul[1].parse::<i64>().ok(), cmul[2].parse::<i64>().ok() ) else {
                        return 0 as i64;
                    };
        
                    first * second
                })
                .sum::<i64>()
        })
        .sum();

    println!("Part2. Result: {result_p2} ");

}
