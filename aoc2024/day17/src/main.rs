mod computer;
use computer::Computer;

mod part1;
mod part2;

fn main() {
    let mut computer = include_str!("input.txt").parse::<Computer>().unwrap();

    // Part 1
    println!("Part 1. Result: {:?}", part1::result(&mut computer));
    
    // Part 2
    let a = part2::result(&mut computer);
    println!("Part 2. Result: {}", a);
    
    // run program with new a
    computer.reset();
    computer.set_reg_a(a);
    computer.run_program();
    println!("Part 2. Program Input : {:?}", computer.get_program());
    println!("Part 2. Program Output: {:?}", computer.get_output());
}
