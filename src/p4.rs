fn number_of_digits(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    return ((n as f64).log(10.0_f64) + 1.0_f64).trunc() as u32;
}

fn get_iest_digit(n: u32, i: u32) -> u32 {
    let divider = 10_u32.pow(i);
    return (n - (n % divider)) / divider % 10 as u32;
}

fn is_palindrome(n: u32) -> bool {
    let len = number_of_digits(n);
    for i in 0..=len / 2 {
        if get_iest_digit(n, i) != get_iest_digit(n, len - i - 1) {
            return false;
        }
    }
    return true;
}

// dummy implementation, generate the largest number possible by the product of the biggest number
// size len, then check if it is a palindrome
fn largest_palindrome_from_product_of(n: u32, len: u32) -> u32 {
    let mut lhs = 10_u32.pow(len);
    let mut rhs = lhs - 1;
    let mut best_palindrome_candidate = 0;
    while lhs > 1 {
        lhs -= 1;
        let palindrome_candidate = lhs * rhs;
        if is_palindrome(palindrome_candidate) {
            best_palindrome_candidate = palindrome_candidate;
            break;
        }
    }
    // from here we found a smallest lhs, let's try to iterate for rhs, maybe we can find something
    // better
    for i in (lhs..(10_u32.pow(len) - 1)).rev() {
        rhs = 10_u32.pow(len); // reset rhs at each lhs i
        while rhs > lhs {
            rhs -= 1;
            let palindrome_candidate = i * rhs;
            if is_palindrome(palindrome_candidate) {
                if palindrome_candidate > best_palindrome_candidate {
                    best_palindrome_candidate = palindrome_candidate;
                }
                // do not continue to iterate through rhs, as we found a palindrome candidate, we
                // need to decrease i to maybe get a new one greater than the current
                // best_palindrome_candidate
                break;
            }
        }
    }
    return best_palindrome_candidate;
}

pub fn problem() {
    println!("A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.");
    println!("Find the largest palindrome made from the product of two 3-digit numbers.");
}

pub fn solve() {
    println!("The largest palindrome made from the product of two 3-digit numbers is: {}", largest_palindrome_from_product_of(2, 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_digits() {
        assert_eq!(number_of_digits(0), 1);
        assert_eq!(number_of_digits(1), 1);
        assert_eq!(number_of_digits(9), 1);
        assert_eq!(number_of_digits(10), 2);
        assert_eq!(number_of_digits(99), 2);
        assert_eq!(number_of_digits(999), 3);
    }

    #[test]
    fn test_get_iest_digit() {
        assert_eq!(get_iest_digit(321, 0), 1);
        assert_eq!(get_iest_digit(321, 1), 2);
        assert_eq!(get_iest_digit(321, 2), 3);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(0));
        assert!(is_palindrome(11));
        assert!(is_palindrome(999));
        assert!(is_palindrome(00));
        assert!(is_palindrome(9119));
        assert!(is_palindrome(9119119));
        assert!(is_palindrome(321123));

        assert!(!is_palindrome(12));
        assert!(!is_palindrome(911919));
        assert!(!is_palindrome(900099));
    }
}
