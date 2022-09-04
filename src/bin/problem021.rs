use std::collections::HashMap;

struct AmicabilityTester {
    memo: HashMap<u32, u32>,
}

impl AmicabilityTester {
    fn new() -> Self {
        AmicabilityTester {
            memo: HashMap::new(),
        }
    }

    fn is_amicable(&mut self, n: u32) -> bool {
        let d = self.d(n);
        if n != d && n == self.d(d) {
            true
        } else {
            false
        }
    }

    fn d(&mut self, n: u32) -> u32 {
        if self.memo.contains_key(&n) {
            return *self.memo.get(&n).unwrap();
        }

        let mut d = 1;
        let mut f = 2;

        loop {
            if n % f == 0 {
                d += f;
                // don't count the square root of n twice
                if d * d != n {
                    d += n / f;
                }
            }
            if f * f >= n {
                break;
            }
            f += 1;
        }

        self.memo.insert(n, d);

        d
    }
}

fn main() {
    let mut tester = AmicabilityTester::new();
    let s: u32 = (1..10000).filter(|n| tester.is_amicable(*n)).sum();
    println!("The sum of amicable numbers under 10000 is: {}", s);
}
