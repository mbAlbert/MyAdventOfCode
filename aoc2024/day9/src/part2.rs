fn encode_filesystem(filesystem: &mut Vec<(Option<usize>, usize)>) {
    for mut i in (0..filesystem.len()).rev() {
        if filesystem[i].0.is_some() {
            let mut none_index = 0;
            while (filesystem[none_index].0.is_some() || filesystem[none_index].1 < filesystem[i].1) && (none_index < i) {
                none_index += 1;
            }

            if none_index >= i {
                continue;
            }

            filesystem[none_index].1 -= filesystem[i].1;
            filesystem.insert(none_index, (None, filesystem[i].1));
            i += 1;
            filesystem.swap(none_index, i);
            
        }
    }
}

fn filesystem_flatten(filesystem: &Vec<(Option<usize>, usize)>) -> Vec<Option<usize>> {
    let mut filesystem_flat = Vec::new();
    for i in 0..filesystem.len() {
        for _ in 0..filesystem[i].1 {
            filesystem_flat.push(filesystem[i].0);
        }
    }

    filesystem_flat
}

fn filesystem_checksum(filesystem: &Vec<(Option<usize>, usize)>) -> usize {
    let filesystem_flat = filesystem_flatten(filesystem);
    let mut checksum = 0;
    for i in 0..filesystem_flat.len() {
        if let Some(value) = filesystem_flat[i] {
            checksum += value * i;
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
        .map(|(index, digit)| {
            let block;
            if index % 2 == 0 {
                block = (Some(id), digit as usize);
                id += 1;
            } else {
                block = (None, digit as usize);
            }

            block
        })
        .collect();

    encode_filesystem(&mut filesystem);
    filesystem_checksum(&filesystem)
}
