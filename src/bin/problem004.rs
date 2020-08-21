// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

use projecteuler::util::is_palindromic;

fn is_product_of_3d_pair(n: u32) -> bool {
    let upper_bound = (n as f32).sqrt().ceil() as u32;
    let mut factor = 100;
    while factor <= upper_bound {
        if n % factor == 0 {
            if n / factor < 1000 && n / factor >= 100 {
                return true;
            }
        }
        factor += 1;
    }
    false
}

fn main() {
    let mut n = 998001;
    loop {
        if is_palindromic(n) && is_product_of_3d_pair(n) {
            break;
        }
        n -= 1;
    }
    println!("The largest palindrome is {}", n);
}
