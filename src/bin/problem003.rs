// What is the largest prime factor of the number 600851475143?

const INPUT_NUMBER: u64 = 600851475143;

fn main() {
    let upper_bound = (INPUT_NUMBER as f64).sqrt().floor() as u64;

    let mut primes: Vec<u64> = vec![2];
    let mut next_prime = 3;
    let mut max_prime_factor = 0;

    let progress_bar = indicatif::ProgressBar::new(upper_bound);

    while next_prime <= upper_bound {
        let mut factor_found = false;
        for p in primes.iter() {
            if next_prime % *p == 0 {
                factor_found = true;
                break;
            }
        }
        if !factor_found {
            primes.push(next_prime);
            if INPUT_NUMBER % next_prime == 0 {
                max_prime_factor = next_prime;
            }
        }
        next_prime += 2;
        progress_bar.inc(2);
    }
    progress_bar.finish();

    println!("The largest prime factor is {}", max_prime_factor);
}
