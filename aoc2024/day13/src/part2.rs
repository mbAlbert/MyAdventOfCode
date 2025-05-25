use crate::claw_machine::ClawMachine;

fn solve_system(machine: &ClawMachine) -> Option<(i64, i64)> {
    // Goal: Solve the system of equations for pa, pb
    // xa*pa + xb*pb = xt
    // ya*pa + yb*pb = yt
    let xa = machine.button_a.0;
    let ya = machine.button_a.1;
    let xb = machine.button_b.0;
    let yb = machine.button_b.1;
    let xt = machine.prize.0 + 10000000000000;
    let yt = machine.prize.1 + 10000000000000;

    // Solve A · x = b as x = A^-1 · b
    // A = [ (xa, ya) (xb, yb) ]
    // x = (pa, pb)
    // b = (xt, yt)
    // A^-1 = 1/det(A) · [ (yb, -xb) (-ya, xa) ]
    let det_a = xa*yb - xb*ya;
    if det_a == 0 {
        return None;
    }

    let numerator_pa = yb*xt - xb*yt;
    let numerator_pb = -ya*xt + xa*yt;

    if ( (numerator_pa % det_a) != 0) || ( (numerator_pb % det_a) != 0) {
        return None;
    } 

    let pa = numerator_pa / det_a;
    let pb = numerator_pb / det_a;
    
    Some((pa, pb))
}

pub fn result(machines: &Vec<ClawMachine>) -> i64 {
    let mut tokens = 0;
    for machine in machines {
        if let Some((pa, pb)) = solve_system(machine) {
            // println!("{:?}", machine);
            // println!("{} {}", pa, pb);
            tokens += 3*pa + pb;
        }
    }

    tokens
}