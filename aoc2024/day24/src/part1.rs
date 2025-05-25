use std::collections::HashMap;

fn backtrace<'a>(
    wires: &mut HashMap<&'a str, Option<u8>>, 
    operations: &HashMap<&'a str, (&'a str, &'a str, &'a str)>, 
    out: &'a str
) {
    
    let &(in1, op, in2) = operations.get(out).unwrap();

    if let Some(None) = wires.get(in1) {
        backtrace(wires, operations, in1);
    }

    if let Some(None) = wires.get(in2) {
        backtrace(wires, operations, in2);
    }

    let in1_v = wires.get(in1).unwrap().unwrap();
    let in2_v = wires.get(in2).unwrap().unwrap();
    let out_v = match op {
        "AND" => in1_v & in2_v,
        "OR" => in1_v | in2_v,
        "XOR" => in1_v ^ in2_v,
        _ => panic!("Invalid operation."),
    };
    wires.insert(out, Some(out_v));
}

pub fn result<'a>(
    wires: &mut HashMap<&'a str, Option<u8>>, 
    operations: &HashMap<&'a str, (&'a str, &'a str, &'a str)>
) -> usize {

    let mut z_out = Vec::new();
    for (&out, _) in operations {
        backtrace(wires, operations, out);
        let out_v = wires.get(out).unwrap().unwrap();
        if out.starts_with('z') {
            z_out.push((out, out_v));
        }
    }
    z_out.sort_by_key(|&(k,_)| k);

    let mut d = 0;
    for (i, (_, z)) in z_out.into_iter().enumerate() {
        d += (z as usize) << i;
    }

    d
}
