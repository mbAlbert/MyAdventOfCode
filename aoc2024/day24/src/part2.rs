use std::collections::{HashMap, HashSet};

fn msb<'a>(operations: &HashMap<&'a str, (&'a str, &'a str, &'a str)>) -> &'a str {
    let mut msb = "z00";

    for (&k,_) in operations {
        if k > msb {
            msb = k;
        }
    }

    msb
}

fn errors<'a>(operations: &HashMap<&'a str, (&'a str, &'a str, &'a str)>, msb: &'a str) -> Vec<&'a str> {
    //-----------------------------------------------
    // Full adder description from input inspection:
    // z00 = x00 ^ y00
    // c00 = x00 & y00
    // zi = xi ^ yi ^ ci, i != 0
    // ci = ((xi ^ yi) & ci)) | (xi & yi), i != 0
    // ----------------------------------------------
    let mut errors = HashSet::new();
    for (&out, &(in1, op, in2)) in operations {
        // Error Case 1. A z bit that is no the most significant one is not formed through an XOR
        if out.starts_with("z") && op != "XOR" && out != msb {
            errors.insert(out);
        }

        // Error Case 2. An XOR operation composed of only intermediate values. XOR always has a x,y,z
        if op == "XOR" 
            && (!in1.starts_with("x") && !in1.starts_with("y") && !in1.starts_with("z"))  
            && (!in2.starts_with("x") && !in2.starts_with("y") && !in2.starts_with("z"))  
            && (!out.starts_with("x") && !out.starts_with("y") && !out.starts_with("z")) 
        {
            errors.insert(out);
        }

        // Error Case 3. The result of an AND is always input into an OR, except for the first carry bit c00. Find an AND that goes to a different gate.
        if op == "AND" && "x00" != in1 && "x00" != in2 {
            for (_, &(n_in1, n_op, n_in2)) in operations {
                if (out == n_in1 || out == n_in2) && n_op != "OR" {
                    errors.insert(out);
                } 
            }
        }

        // Error Case 4. The result of an XOR is always input into an AND or XOR. Find an XOR that goest to an OR.
        if op == "XOR" {
            for (_, &(n_in1, n_op, n_in2)) in operations {
                if (out == n_in1 || out == n_in2) && n_op == "OR" {
                    errors.insert(out);
                } 
            }
        }
    }

    errors.into_iter().collect()
}

pub fn result<'a>(operations: &HashMap<&'a str, (&'a str, &'a str, &'a str)>) -> Vec<&'a str> {

    let msb = msb(operations);
    let mut errors = errors(operations, msb);
    errors.sort();

    errors
}
