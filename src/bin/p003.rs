fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        x if is_prime(x) => x,
        _ => (2..n)
            .into_iter()
            .take_while(|&d| d * d <= n)
            .filter(|&d| n % d == 0 && is_prime(d))
            .last()
            .unwrap(),
    }
}

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _ => {
            (2..n)
                .into_iter()
                .take_while(|&d| d * d <= n)
                .filter(|&d| n % d == 0)
                .count()
                == 0
        }
    }
}
