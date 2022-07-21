// There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

fn main() {
    for m in 1u64.. {
        if 500 % m == 0 {
            let n = 500 / m - m;
            if n > m {
                continue;
            }
            if 2 * m * (m + n) == 1000 {
                let m4 = m * m * m * m;
                let n4 = n * n * n * n;
                println!("The product is {}", (m4 - n4) * 2 * m * n);
                break;
            }
        }
    }
}
