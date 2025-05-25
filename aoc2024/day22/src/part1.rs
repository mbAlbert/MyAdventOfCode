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

fn next_n(secret: usize, n: usize) -> usize {
    if n == 0 {
        return secret;
    }

    return next_n(next_secret(secret), n - 1);
}

pub fn result() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| next_n(line.parse::<usize>().unwrap(), 2000))
        .sum::<usize>()
}
