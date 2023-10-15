use std::{hash::Hash, collections::HashSet};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Throw {
    Single(u8),
    Double(u8),
    Treble(u8),
    Bull,
    DoubleBull,
}

impl Throw {
    fn score(&self) -> u8 {
        match self {
            Throw::Single(n) => *n,
            Throw::Double(n) => *n * 2,
            Throw::Treble(n) => *n * 3,
            Throw::Bull => 25,
            Throw::DoubleBull => 50,
        }
    }

    fn get_all_possible() -> Vec<Throw> {
        let mut result = Vec::new();
        for n in 1..=20 {
            result.push(Throw::Single(n));
            result.push(Throw::Double(n));
            result.push(Throw::Treble(n));
        }
        result.push(Throw::Bull);
        result.push(Throw::DoubleBull);
        result
    }
}

#[derive(Clone, Debug)]
struct Checkout {
    checkout: Vec<Throw>,
}

impl Checkout {
    fn score(&self) -> u8 {
        self.checkout.iter().map(|t| t.score()).sum()
    }

    fn new() -> Self {
        Checkout { checkout: Vec::new() }
    }

    fn push(&mut self, t: Throw) {
        self.checkout.push(t);
    }

    fn normalized(&self) -> Self {
        let mut result = self.clone();
        let last = result.checkout.pop();
        result.checkout.sort();
        if let Some(t) = last {
            result.checkout.push(t);
        }
        result
    }
}

impl Eq for Checkout {
}

impl PartialEq for Checkout {
    fn eq(&self, other: &Self) -> bool {
        let s = self.normalized();
        let o = other.normalized();
        s.checkout.eq(&o.checkout)
    }
}

impl Hash for Checkout {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let normalized = self.normalized();
        normalized.checkout.hash(state);
    }
}

fn get_checkouts(partial: &Checkout, mut result: HashSet<Checkout>) -> HashSet<Checkout> {
    if partial.checkout.len() < 3 {
        for throw in Throw::get_all_possible() {
            let mut candidate = partial.clone();
            candidate.push(throw.clone());
            if candidate.score() < 100 {
                result = get_checkouts(&candidate, result);
                if let Throw::Double(_) = throw {
                    result.insert(candidate);
                } else if let Throw::DoubleBull = throw {
                    result.insert(candidate);
                }
            }
        }
    }
    result
}

fn main() {
    let count = get_checkouts(&Checkout::new(), HashSet::new()).len();
    println!("Number of possible checkouts under 100: {}", count);
}
