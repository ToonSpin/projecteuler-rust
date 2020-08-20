// Each new term in the Fibonacci sequence is generated by adding the previous
// two terms. By starting with 1 and 2, the first 10 terms will be:
//
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values do not exceed
// four million, find the sum of the even-valued terms.

fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;

    while b <= 4_000_000u32 {
        if b % 2 == 0 {
            sum += b;
        }
        let t = a + b;
        a = b;
        b = t;
    }

    println!("The sum of the even Fibonacci terms is: {}", sum);
}