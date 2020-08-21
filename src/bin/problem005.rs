// 2520 is the smallest number that can be divided by each of the numbers from 1
// to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the
// numbers from 1 to 20?

use std::collections::HashMap;

use projecteuler::util::primer::Primer;

fn main() {
    let primer = Primer::init_with_max_value(20u64);
    let primes = primer.get_primes();
    let mut min_factors: HashMap<u64, u64> = HashMap::new();

    for p in primes.iter() {
        min_factors.insert(*p, 0);
    }

    for factor in 2..=20 {
        let mut n = factor;

        for p in primes.iter() {
            let mut count = 0;

            while n % *p == 0 {
                count += 1;
                n /= *p;
            }

            let current_min_factor = min_factors.get(&*p).unwrap();
            if *current_min_factor < count {
                min_factors.insert(*p, count);
            }
        }
    }

    let mut product = 1;

    for (prime, exponent) in min_factors.iter() {
        for _i in 0..*exponent {
            product *= prime;
        }
    }

    println!("The smallest number divisible by 1..20 is {}", product);
}
