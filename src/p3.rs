pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in 3..limit {
        if i % 2 != 0 {
            if n % i == 0 {
                return false;
            }
        }
    }
    return true;
}

fn largest_prime_factor_of(n: u64) -> u64 {
    if is_prime(n) {
        return n;
    }
    let to_check = (n as f64).sqrt() as u64;
    for i in (1..=to_check).rev() {
        if is_prime(i) && n % i == 0 {
            return i;
        }
    }
    return n;
}

pub fn problem() {
    println!("The prime factors of 13195 are 5, 7, 13 and 29.");
    println!("What is the largest prime factor of the number 600851475143 ?");
}

pub fn solve() {
    println!("The largest prime factor of the number 600851475143 is: {}", largest_prime_factor_of(600851475143));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(13));

        assert!(!is_prime(4));
        assert!(!is_prime(15));
        assert!(!is_prime(9));
        assert!(!is_prime(21));
    }

    #[test]
    fn test_largest_prime_factor_of() {
        assert_eq!(largest_prime_factor_of(2), 2);
        assert_eq!(largest_prime_factor_of(3), 3);
        assert_eq!(largest_prime_factor_of(5), 5);
        assert_eq!(largest_prime_factor_of(4), 2);
        assert_eq!(largest_prime_factor_of(9), 3);
    }
}
