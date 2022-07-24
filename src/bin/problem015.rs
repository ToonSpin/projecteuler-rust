// Starting in the top left corner of a 2×2 grid, and only being able to move to
// the right and down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?

use std::collections::HashMap;

fn lattice_paths(size: (u64, u64), memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    let (w, h) = size;
    if w < h {
        return lattice_paths((h, w), memo);
    }
    if h == 0 {
        return 1;
    }
    if h == 1 {
        return w + 1;
    }
    if memo.contains_key(&size) {
        return *memo.get(&size).unwrap();
    }
    let result = lattice_paths((w - 1, h), memo) + lattice_paths((w, h - 1), memo);
    memo.insert(size, result);
    result
}

fn main() {
    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();
    println!("The number of routes through a 20×20 grid is: {}", lattice_paths((20, 20), &mut memo));
}
