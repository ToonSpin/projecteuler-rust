// Find the sum of all the primes below two million.

use projecteuler::util::primer::Primer;

fn main() {
    let primer: Primer<u64> = Primer::new();
    let sum: u64 = primer.iter().take_while(|&i| i < 2_000_000).sum();
    println!("The sum of all the primes below two million is: {}", sum);
}
