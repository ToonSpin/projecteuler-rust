use projecteuler::util::digitalnumber::DigitalNumber;

fn main() {
    let mut product = DigitalNumber::from(1);
    for factor in 2..=100u32 {
        product = product * factor;
    }
    let sum: u32 = product.get_digits().iter().map(|d| *d as u32).sum();
    println!("The sum of the digits of 100! is: {}", sum);
}
