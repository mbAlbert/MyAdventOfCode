pub fn blink(stones: &Vec<u64>, n: usize) -> usize {
    let mut stones = stones.clone();
    for _ in 0..n {
        let mut stones_next = Vec::new();
        for i in 0..stones.len() {
            if stones[i] == 0 {
                stones_next.push(1 as u64);
            } else {
                let digits = stones[i].ilog10() + 1;
                if digits % 2 == 0 {
                    let div = 10_u64.pow(digits/2);
                    stones_next.push(stones[i] / div);
                    stones_next.push(stones[i] % div);
                } else {
                    stones_next.push(stones[i] * 2024);
                }
            }
        }

        stones = stones_next;
    }

    stones.len()
}