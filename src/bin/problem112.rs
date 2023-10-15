use std::str::FromStr;
use projecteuler::util::digitalnumber::DigitalNumber;

fn is_bouncy(n: &DigitalNumber) -> bool {
    if n.count_digits() < 3 {
        return false;
    }
    let digits = n.get_digits();
    let mut is_increasing = false;
    let mut is_decreasing = false;

    for i in 1..digits.len() {
        if digits[i - 1] < digits[i] {
            // logic looks backwards, but the digits are reversed
            is_decreasing = true;
            if is_increasing {
                return true;
            }
        }
        if digits[i - 1] > digits[i] {
            // logic looks backwards, but the digits are reversed
            is_increasing = true;
            if is_decreasing {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut n = DigitalNumber::from_str("1").unwrap();
    let mut nonbouncy_count = 0;
    let mut total_count = 0;
    loop {
        if !is_bouncy(&n) {
            nonbouncy_count += 1;
        }
        total_count += 1;
        n.increment();
        if nonbouncy_count * 100 == total_count {
            break;
        }
    }
    println!("Exactly 99% of the first {} numbers are bouncy", total_count);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_problem_examples() {
        let example = DigitalNumber::from_str("134468").unwrap();
        assert!(!is_bouncy(&example));

        let example = DigitalNumber::from_str("66420").unwrap();
        assert!(!is_bouncy(&example));

        let example = DigitalNumber::from_str("155349").unwrap();
        assert!(is_bouncy(&example));
    }
}
