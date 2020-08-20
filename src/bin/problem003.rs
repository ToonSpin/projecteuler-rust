// What is the largest prime factor of the number 600851475143?

const INPUT_NUMBER: u64 = 600851475143;

fn main() {
    let mut primes: Vec<u64> = vec![2];
    let mut next_prime = 3;
    let mut max_prime_factor = 0;
    let mut n = INPUT_NUMBER;

    while n > 1 {
        let mut not_prime = false;

        for p in primes.iter() {
            if next_prime % *p == 0 {
                not_prime = true;
                break;
            }
        }
        if !not_prime {
            primes.push(next_prime);
            if n % next_prime == 0 {
                max_prime_factor = next_prime;
                n /= max_prime_factor;
            }
        }
        next_prime += 2;
    }

    println!("The largest prime factor is {}", max_prime_factor);
}
