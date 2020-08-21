use projecteuler::util::Primer;

fn main() {
    let primer: Primer<u32> = Primer::new();
    println!("The 10001st prime is {}", primer.iter().nth(10000).unwrap());
}
