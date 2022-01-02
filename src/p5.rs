fn smallest_positive_number_evenly_divisible_by_to(n: u32) -> u32 {
    let mut found = false;
    let mut candidate = n;
    if candidate % 2 != 0 && candidate > 2 {
        candidate += 1; // only test even numbers
    }
    while !found {
        found = true;
        candidate += 2; // avoid odd numbers
        for i in 1..n {
            if candidate % i != 0 {
                found = false;
                break;
            }
        }
    }
    return candidate;
}

pub fn problem() {
    println!("2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.");
    println!("What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?");
}

pub fn solve() {
    println!("The smallest positive number evenly divisible by all of the numbers from 1 to 20 is: {}", smallest_positive_number_evenly_divisible_by_to(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_positive_number_evenly_divisible_by_to() {
        assert_eq!(smallest_positive_number_evenly_divisible_by_to(10), 2520);
    }
}
