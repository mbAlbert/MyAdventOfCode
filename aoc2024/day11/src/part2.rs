use std::collections::HashMap;

fn cached_stone_count(cache: &mut HashMap<(u64, usize), usize>, stone: u64, remaining: usize) -> usize {
    if let Some(&count) = cache.get(&(stone, remaining)) {
        return count;
    }

    let count = stone_count(cache, stone, remaining);
    cache.insert((stone, remaining), count);
    return count;
}

fn stone_count(cache: &mut HashMap<(u64, usize), usize>, stone: u64, remaining: usize) -> usize {
    if remaining == 0 {
        return 1;
    }

    if stone == 0 {
        return cached_stone_count(cache, 1, remaining - 1);
    } else {
        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let div = 10_u64.pow(digits/2);
            return cached_stone_count(cache, stone / div, remaining - 1) + cached_stone_count(cache, stone % div, remaining - 1);
        } else {
            return cached_stone_count(cache, stone * 2024, remaining - 1);
        }
    }
}

pub fn blink(stones: &Vec<u64>, n: usize) -> usize {
    let mut count = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        count += stone_count(&mut cache, *stone, n);
    }

    count
}