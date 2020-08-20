// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    let mut sum = 0;

    for i in 1..1000 {
        if i % 3 == 0 {
            sum += i;
            continue;
        }
        if i % 5 == 0 {
            sum += i;
        }
    }

    println!("The sum of multiples of 3 or 5 below 1000 is {}", sum);
}
