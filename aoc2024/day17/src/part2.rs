use crate::computer::Computer;

// Explicit input for faster program iteratotions
fn iterate(a: usize) -> usize {
    let b = a & 7 ^ 5;
    let c = a / (1 << b);
    let b = b ^ c ^ 6;
    b & 7
}

fn find_a(program: &Vec<u8>) -> Option<usize> {
    let mut stack = Vec::new();
    stack.push((0, program.len() - 1));

    while let Some((prev_a, col)) = stack.pop() {
        for i in (0..8).rev() {     
            let a = prev_a * 8 + i;
            if iterate(a) as u8 == program[col] {
                if col == 0 {
                    return Some(a);
                }

                stack.push((a, col - 1));
            }
        }
    }
    None
}

pub fn result(computer: &mut Computer) -> usize {
    let program = computer.get_program();
    let a = find_a(&program).unwrap();

    a
}