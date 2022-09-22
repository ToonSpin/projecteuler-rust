use projecteuler::util::digitalnumber::DigitalNumber;

fn main() {
    let mut a = DigitalNumber::from(1);
    let mut b = DigitalNumber::from(1);
    let mut index = 3;

    loop {
        let c = a + b.clone();
        if c.count_digits() >= 1000 {
            break;
        }
        a = b;
        b = c;
        index += 1;
    } 

    println!("The index of the first Fibonacci number with at least 1000 digits is: {}", index);
}
