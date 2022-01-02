use crate::p3::is_prime;

fn iest_prime_number(n: u32) -> u64 {
    let mut candidate = 1;
    let mut i = 0;
    while i < n {
        candidate += 1;
        while !is_prime(candidate) {
            candidate += 1;
        }
        i += 1;
    }
    return candidate;
}

pub fn problem() {
    println!("By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.");
    println!("What is the 10 001st prime number?");
}

pub fn solve() {
    println!("The 10 001st prime number is: {}", iest_prime_number(10001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iest_prime_number() {
        assert_eq!(iest_prime_number(1), 2);
        assert_eq!(iest_prime_number(6), 13);
    }
}
