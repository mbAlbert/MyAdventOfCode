use std::collections::HashMap;

fn mix(secret: usize, mix: usize) -> usize {
    secret ^ mix
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next_secret(secret: usize) -> usize {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}

fn vendor_numbers(secret: usize, n: usize) -> Vec<usize> {
    let mut secret_numbers = Vec::with_capacity(n + 1);
    secret_numbers.push(secret);
    for i in 1..=n { 
        let next = next_secret(secret_numbers[i-1]); 
        secret_numbers.push(next);
    }

    secret_numbers
}

fn vendor_prices(numbers: &Vec<usize>) -> Vec<usize> {
    numbers.into_iter().map(|&n| n % 10).collect()
}

fn vendor_deltas(prices: &Vec<usize>) -> Vec<isize>{
    prices.windows(2).map(|w| w[1] as isize - w[0] as isize).collect()
}

fn vendor_map(prices: &Vec<usize>, deltas: &Vec<isize>) -> HashMap<(isize, isize, isize, isize), usize> {
    let mut map = HashMap::new();

    for (i, w) in deltas.windows(4).enumerate() {
        if !map.contains_key(&(w[0], w[1], w[2], w[3])) {
            map.insert((w[0], w[1], w[2], w[3]), prices[i+4]);
        }
    }

    map
}

pub fn result() -> usize {
    let vendors: Vec<_> = include_str!("input.txt").lines().map(|line| line.parse::<usize>().unwrap()).collect();

    let mut bananas = HashMap::new();
    for seed in vendors {
        let numbers = vendor_numbers(seed, 2000);
        let prices = vendor_prices(&numbers);
        let deltas = vendor_deltas(&prices);
        let map = vendor_map(&prices, &deltas);

        for (key, value) in map {
            bananas.entry(key).and_modify(|v| *v += value as usize).or_insert(value as usize);
        }
    }

    *bananas.iter().max_by_key(|&(_, v)| v).unwrap().1
}
