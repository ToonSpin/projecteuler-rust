fn recurring_cycle_length(n: u32) -> usize {
    let mut remainders = Vec::new();
    let found: usize;
    let mut d = 1;
    loop {
        d *= 10;
        let r = d % n;
        if let Some(n) = remainders.iter().position(|&x| x == r) {
            found = n;
            break;
        }
        remainders.push(r);
        d = r;
    }
    remainders.len() - found
}

fn main() {
    let mut max_number = 0;
    let mut max_length = 0;
    for n in 2..1000 {
        let length = recurring_cycle_length(n);
        if length > max_length {
            max_number = n;
            max_length = length;
        }
    }
    println!("The number with the longest recurring cycle length is: {}", max_number);
}
