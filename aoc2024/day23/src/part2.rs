use std::collections::{HashMap, HashSet};

pub fn result(map: &HashMap<&str, HashSet<&str>>) -> Vec<String> {
    let mut max = 0;
    let mut result= Vec::new();
    for (r1, pool1) in map.clone() {
        for r2 in pool1.clone() {
            let pool2 = map.get(r2).unwrap();
            let intersect: HashSet<_> = pool1.intersection(pool2).collect();
            if intersect.len() > 0 {
                let intersect: Vec<_> = intersect.into_iter().collect();
                let mut count = 0;
                let mut routers = HashSet::new();
                routers.insert(r1.to_string());
                routers.insert(r2.to_string());
                for i in 0..intersect.len() {
                    for j in i+1..intersect.len() {
                        let i_set =  map.get(intersect[i]).unwrap();
                        if i_set.contains(intersect[j]) {
                            count += 1;
                            routers.insert(intersect[i].to_string());
                            routers.insert(intersect[j].to_string());
                        }
                    }
                }

                if count > max {
                    max = count;
                    result = routers.into_iter().collect();
                }
            }
        }


    }

    result.sort();

    result
}