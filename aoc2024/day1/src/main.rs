fn get_input_lists() -> (Vec<u64>, Vec<u64>) {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut ids = line.split_whitespace();
            let (Some(id1), Some(id2)) = (ids.next(), ids.next()) else {
                panic!("Invalid input...");
            };

            (id1.parse::<u64>().unwrap(), id2.parse::<u64>().unwrap())
        })
        .unzip()
}

fn get_distance(left_list: &mut Vec<u64>, right_list: &mut Vec<u64>) -> u64 {
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&id_l, &id_r)| (id_l.abs_diff(id_r)))
        .sum()
}

fn get_similarity_score(left_list: &mut Vec<u64>, right_list: &mut Vec<u64>) -> u64 {
    left_list
        .iter()
        .map( |&id_l| ( right_list.iter().filter(|&&id_r| id_l == id_r).count() as u64) * id_l )
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut left_list, mut right_list) = get_input_lists();
    
    // Part1
    let distance = get_distance(&mut left_list, &mut right_list);
    println!("Part1.\n The total distance is: {distance}\n");

    // Part 2
    let similarity_score = get_similarity_score(&mut left_list, &mut right_list);
    println!("Part 2.\n The total similarity score is: {similarity_score}");

    Ok(())
}
