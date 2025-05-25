fn encode_filesystem(filesystem: &mut Vec<Option<usize>>) {
    let mut none_index = 0;

    for i in (0..filesystem.len()).rev() {
        if let Some(value) = filesystem[i] {
            while filesystem[none_index].is_some() {
                none_index += 1;
            }

            if none_index >= i {
                break;
            }

            filesystem[none_index] = Some(value);
            filesystem[i] = None;
        }
    }
}

fn filesystem_checksum(filesystem: &Vec<Option<usize>>) -> usize {
    let mut checksum = 0;
    for i in 0..filesystem.len() {
        if let Some(value) = filesystem[i] {
            checksum += value * i;
        } else {
            break;
        }
    }

    checksum
}

pub fn result() -> usize {
    let mut id: usize = 0;
    let mut filesystem: Vec<_> = include_str!("input.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(|(index, digit)| {
            let block;
            if index % 2 == 0 {
                block = std::iter::repeat(Some(id)).take(digit as usize);
                id += 1;
            } else {
                block = std::iter::repeat(None).take(digit as usize);
            }

            block
        })
        .collect();

    encode_filesystem(&mut filesystem);
    filesystem_checksum(&filesystem)
}
