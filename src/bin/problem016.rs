// What is the sum of the digits of the number 2^1000?

fn double(mut number: Vec<u32>) -> Vec<u32> {
    let mut carry = false;
    for d in number.iter_mut() {
        *d *= 2;
        if carry {
            *d += 1;
        }
        if *d >= 10 {
            *d -= 10;
            carry = true;
        } else {
            carry = false;
        }
    }
    if carry {
        number.push(1);
    }
    number
}

fn main() {
    let mut number = vec![2];
    for _i in 1..1000 {
        number = double(number);
    }
    let sum: u32 = number.iter().sum();
    println!("The sum of the digits of 2^1000 is: {}", sum);
}
