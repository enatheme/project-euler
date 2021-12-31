fn fibonacci(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn fibonacci_even_sum(limit: u32) -> u32 {
    let mut i = 1;
    let mut j = 1;
    let mut sum = 0;
    while i < limit {
        i = i + j;
        j = i - j;
        if i % 2 == 0 {
            sum += i;
        }
    }
    return sum;
}

pub fn problem() {
    println!("Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:");
    println!("1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...");
    println!("By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.");
}

pub fn solve() {
    println!("Sum of the even-valued terms lower than 4 million: {}", fibonacci_even_sum(4000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(6), 13);
        assert_eq!(fibonacci(7), 21);
        assert_eq!(fibonacci(8), 34);
        assert_eq!(fibonacci(9), 55);
        assert_eq!(fibonacci(10), 89);
    }

    #[test]
    fn test_fibonacci_even_sum() {
        let mut sum = 0;
        let limit = 10;
        for i in 1..limit {
            let fib = fibonacci(i);
            if fib > limit {
                break;
            }
            if fib % 2 == 0 {
                sum += fib;
            }
        }
        assert_eq!(sum, fibonacci_even_sum(limit));
    }
}
