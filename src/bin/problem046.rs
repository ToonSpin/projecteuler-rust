use projecteuler::util::primer::Primer;

fn is_goldbach_or_prime(n: u32) -> bool {
    let mut m = 1;
    let primer = Primer::init_with_max_value(n);
    let primes = primer.get_primes();
    if let Ok(_) = primes.binary_search(&n) {
        return true;
    }
    while m * m * 2 < n {
        let candidate = m * m * 2;
        if let Ok(_) = primes.binary_search(&(n - candidate)) {
            return true;
        }
        m += 1;
    }
    false
}

fn main() {
    let mut candidate = 9;
    while is_goldbach_or_prime(candidate) {
        candidate += 2;
    }
    println!("The smallest odd composite that doesn't conform: {}", candidate);
}
