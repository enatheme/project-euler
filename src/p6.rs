fn sum_to(n: u32) -> u32 {
    n * (n + 1) / 2
}

fn sum_squares_to(n: u32) -> u32 {
    let mut ret = 0;
    for i in 1..=n {
        ret += i.pow(2);
    }
    ret
}

fn difference_sum_of_squares_and_squares_of_sum_of(n: u32) -> u32 {
    let sum_of_the_suqares = sum_squares_to(n);
    let square_of_the_sum = sum_to(n).pow(2);
    square_of_the_sum - sum_of_the_suqares
}

pub fn problem() {
    println!("The sum of the squares of the first ten natural numbers is,");
    println!("1^2 + 2^2 + ... + 10^2 = 385");
    println!("The square of the sum of the first ten natural numbers is,");
    println!("(1 + 2 + ... + 10)^2 = 55^2 = 3025");
    println!("Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.");
    println!("Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");
}

pub fn solve() {
    println!("The difference between the sum of the squares of the first 100 hundred natural numbers and the square of the sum is: {}", difference_sum_of_squares_and_squares_of_sum_of(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to() {
        assert_eq!(sum_to(1), 1);
        assert_eq!(sum_to(2), 3);
        assert_eq!(sum_to(20), 210);
    }

    #[test]
    fn test_sum_squares_to() {
        assert_eq!(sum_squares_to(10), 385);
    }

    #[test]
    fn test_difference_sum_of_squares_and_squares_of_sum_of() {
    }
}
