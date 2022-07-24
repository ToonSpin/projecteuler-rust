use std::collections::hash_map::Entry::Occupied;
use std::collections::HashMap;

fn collatz(start: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if start == 1 {
        return 1;
    }
    if let Occupied(n) = memo.entry(start) {
        return *n.get();
    }
    let result = 1 + if start % 2 == 0 {
        collatz(start / 2, memo)
    } else {
        collatz(3 * start + 1, memo)
    };
    memo.insert(start, result);
    result
}

fn main() {
    let mut memo: HashMap<u64, u64> = HashMap::new();
    let mut max_start = 0;
    let mut max_terms = 0;

    for start in 1..1_000_000 {
        let terms = collatz(start, &mut memo);
        if terms > max_terms {
            max_start = start;
            max_terms = terms;
        }
    }
    println!("The starting number with the longest chain is: {}", max_start);
}
