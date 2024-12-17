mod computer;
use computer::Computer;

fn part1(computer: &mut Computer) {
    computer.run_program();
    println!("Part 1. {:?}", computer.get_output());
}

fn part2(computer: &mut Computer) {
    let program = computer.get_program();
    let a = find_a(&program).unwrap();

    // run program with new a
    computer.reset();
    computer.set_reg_a(a);
    computer.run_program();
    println!("Part 2. Program: {:?}", program);
    println!("Part 2. Output : {:?}", computer.get_output());

    // show result
    println!("Part 2. a: {}", a);
}

// Decompile input for faster find_a
fn iterate(a: usize) -> usize {
    let b = a & 7 ^ 5;
    let c = a / (1 << b);
    let b = b ^ c ^ 6;
    b & 7
}

fn find_a(program: &Vec<u8>) -> Option<usize> {
    let mut queue = std::collections::VecDeque::new();
    queue.push_front((0, program.len() - 1));

    while let Some((prev_a, col)) = queue.pop_back() {
        for i in 0..8 {     
            let a = prev_a * 8 + i;
            if iterate(a) as u8 == program[col] {
                if col == 0 {
                    return Some(a);
                }

                queue.push_front((a, col - 1));
            }
        }
    }
    None
}

fn main() {
    let mut computer = include_str!("input.txt").parse::<Computer>().unwrap();
    part1(&mut computer);
    part2(&mut computer);
}
