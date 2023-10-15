use projecteuler::util::primer;

// https://math.stackexchange.com/a/3719034
fn count_reciprocals(n: u64, primer: &mut primer::Primer<u64>) -> u64 {
    let factorization = primer::prime_factorization(n * n, primer);
    let factors = primer::get_all_factors_from_prime_factors(&factorization);
    let mut count = 0;
    for a in factors.iter() {
        let x = a + n;
        let b = n * n / a;
        let y = b + n;

        if x > y {
            // factors isn't sorted. Benchmarks show that sorting it and
            // breaking here is slower than not sorting and continuing.
            continue;
        }

        if x * y == n * (x + y) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut primer: primer::Primer<u64> = primer::Primer::new();
    let mut n = 0;

    loop {
        n += 1;
        let count = count_reciprocals(n, &mut primer);
        if count > 1000 {
            println!("The number of reciprocals > 1000 for: {}", n);
            break;
        }
    }
}
