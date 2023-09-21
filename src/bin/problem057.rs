use projecteuler::util::digitalnumber::DigitalNumber;

type Fraction = (DigitalNumber, DigitalNumber);

fn iterate(f: Fraction) -> Fraction {
    let (mut numerator, mut denominator) = f;
    numerator -= &denominator;

    numerator += &denominator;
    numerator += &denominator;

    let temp = denominator;
    denominator = numerator;
    numerator = temp;
    numerator += &denominator;

    (numerator, denominator)
}

fn main() {
    let mut f = (DigitalNumber::from(3), DigitalNumber::from(2));
    let mut count = 0;
    for _ in 0..1000 {
        let (numerator, denominator) = &f;
        if numerator.count_digits() > denominator.count_digits() {
            count += 1;
        }
        f = iterate(f);
    }
    println!("The number of iterations: {}", count);
}
