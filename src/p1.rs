fn is_multiple(multiple: u32, divider: u32) -> bool {
    multiple % divider == 0
}

fn is_multiple_of(multiple: u32, array_of_divider: &[u32]) -> bool {
    for divider in array_of_divider.iter() {
        if is_multiple(multiple, *divider) {
            return true;
        }
    }
    return false;
}

fn sum_of_all_multiple_of_bellow(to: u32, array_of_divider: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 1..to {
        if is_multiple_of(i, &array_of_divider) {
            sum += i;
        }
    }
    sum
}

pub fn problem() {
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.");
    println!("Find the sum of all the multiples of 3 or 5 below 1000.");
}

pub fn solve() {
    println!("Sum of all multiples of 3 or 5 bellow 1000: {}", sum_of_all_multiple_of_bellow(1000, &[3, 5]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_multiple() {
        assert!(is_multiple(9, 3));
        assert!(is_multiple(9, 1));
        assert!(!is_multiple(9, 5));
        assert!(!is_multiple(3, 9));
    }

    #[test]
    fn test_is_multiple_of() {
        assert!(is_multiple_of(9, &[3, 5]));
        assert!(is_multiple_of(10, &[3, 5]));
        assert!(!is_multiple_of(7, &[3, 5]));
        assert!(!is_multiple_of(8, &[3, 5]));
    }

    #[test]
    fn test_sum_of_all_multiple_of_bellow() {
        assert_eq!(23, sum_of_all_multiple_of_bellow(10, &[3, 5]));
        assert_eq!(33, sum_of_all_multiple_of_bellow(11, &[3, 5]));
    }
}
