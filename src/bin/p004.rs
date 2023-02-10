fn main() {
    println!("{}", solution());
}

fn solution() -> u64 {
    ((100 * 100)..(999 * 999))
        .into_iter()
        .rev()
        .filter(|&val| is_palindrome(val) && is_product(val))
        .next()
        .unwrap()
}

fn is_product(n: u64) -> bool {
    match n {
        x if x < 100 * 100 || x > 999 * 999 => false,
        _ => {
            (100..1000)
                .into_iter()
                .filter(|&d| n % d == 0 && n / d >= 100 && n / d <= 999)
                .count()
                > 0
        }
    }
}

fn is_palindrome(n: u64) -> bool {
    let number = n.to_string();
    let n_digits = number.len();
    return number[0..(n_digits / 2)]
        == number.chars().rev().collect::<String>()[0..(n_digits / 2)];
}


#[cfg(test)]
mod tests {
    use crate::{is_product, is_palindrome};

    #[test]
    fn test_product() {
        assert_eq!(is_product(0), false);
        assert_eq!(is_product(100 * 50), false);
        assert_eq!(is_product(100 * 100), true);
        assert_eq!(is_product(700 * 200), true);
        assert_eq!(is_product(999 * 999), true);
        assert_eq!(is_product(1000 * 1000), false);
    }

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome(0), true);
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(5), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(1991), true);
    }
}
