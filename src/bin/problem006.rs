// Find the difference between the sum of the squares of the first one hundred
// natural numbers and the square of the sum.

fn main() {
    let mut sum = 0;
    let mut sum_of_squares = 0;

    for i in 1..=100u64 {
        sum += i;
        sum_of_squares += i * i;
    }

    println!(
        "The difference between the square of the sum and the sum of the squares is {}",
        sum * sum - sum_of_squares
    );
}
