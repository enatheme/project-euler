fn is_perfect_square(n: f64) -> bool {
    let divider = n.sqrt();
    divider.floor() == divider.ceil()
}

fn pythagorean_triplet(n: u32) -> u32 {
    let mut a: u32 = 0;
    loop {
        for b in 0..a {
            let c_sq = (a.pow(2) + b.pow(2)) as f64;
            if is_perfect_square(c_sq) {
                // as c_sq is perfect square, than it's safe to trunk the sqrt
                let c = c_sq.sqrt().floor() as u32;
                if a + b + c == n {
                    return a * b * c;
                }
            }
        }
        a += 1;
    }
}

pub fn problem() {
    println!("A Pythagorean triplet is a set of three natural numbers, a < b < c, for which, a^2 + b^2 = c^2");
    println!("For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.");
    println!("There exists exactly one Pythagorean triplet for which a + b + c = 1000.");
    println!("Find the product abc.");
}

pub fn solve() {
    println!("The Pythagorean triplet a * b * c where a + b + c = 1000 is: {}", pythagorean_triplet(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert!(is_perfect_square(4_f64));
        assert!(is_perfect_square(25_f64));
        assert!(is_perfect_square(9_f64));

        assert!(!is_perfect_square(5_f64));
        assert!(!is_perfect_square(15_f64));
        assert!(!is_perfect_square(21_f64));
        assert!(!is_perfect_square(12_f64));
    }
}
