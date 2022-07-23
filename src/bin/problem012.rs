// What is the value of the first triangle number to have over five hundred divisors?

use projecteuler::util::primer::{Primer, prime_factorization};

fn num_factors(n: u32, primer: &mut Primer<u32>) -> usize {
    let mut count = 1;
    for (_f, n) in prime_factorization(n, primer) {
        count *= n + 1;
    }
    count
}

fn main() {
    let mut primer = Primer::new();
    let mut triangle = 0;

    for n in 1.. {
        triangle += n;
        let nf = num_factors(triangle, &mut primer);
        if nf > 500 {
            println!("The first triangle number with more than 500 divisors is: {}", triangle);
            break;
        }
    }
}
