use std::collections::{HashMap, HashSet};

pub fn result(map: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut connections = HashSet::new();
    for (r1, pool1) in map.clone() {
        for r2 in pool1.clone() {
            let pool2 = map.get(r2).unwrap();
            let intersect: HashSet<_> = pool1.intersection(pool2).collect();
            if intersect.len() > 0 {
                for &r3 in intersect {
                    if r1.starts_with('t') || r2.starts_with('t') || r3.starts_with('t') {
                        let mut routers = [r1, r2, r3];
                        routers.sort();
                        connections.insert(routers);
                    } 
                } 
            }
        }
    }

    // for connection in connections {
    //     println!("{:?}", connection);
    // }

    connections.len()
}