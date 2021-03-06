use crate::util::UnsignedInt;

pub struct Primer<T: UnsignedInt> {
    primes: Vec<T>,
    iter_index: usize,
}

impl<T: UnsignedInt> Primer<T> {
    pub fn new() -> Primer<T> {
        Primer {
            primes: Vec::new(),
            iter_index: 0,
        }
    }

    fn prime_test_against_vec(n: T, primes: &Vec<T>) -> bool {
        for p in primes.iter() {
            if *p * *p > n {
                break;
            }
            if n % *p == 0.into() {
                return false;
            }
        }
        true
    }

    pub fn init_with_max_value(max_value: T) -> Primer<T> {
        let mut primes: Vec<T> = vec![2.into(), 3.into()];
        let mut next_prime: T = 5.into();

        while next_prime <= max_value {
            if Self::prime_test_against_vec(next_prime, &primes) {
                primes.push(next_prime);
            }
            next_prime = next_prime + 2.into();

            if Self::prime_test_against_vec(next_prime, &primes) {
                primes.push(next_prime);
            }
            next_prime = next_prime + 4.into();
        }

        Primer {
            primes,
            iter_index: 0,
        }
    }

    pub fn get_primes(self) -> Vec<T> {
        self.primes.clone()
    }

    pub fn iter(self) -> Primer<T> {
        self
    }
}

impl<T: UnsignedInt> std::iter::Iterator for Primer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.iter_index < self.primes.len() {
            self.primes[self.iter_index]
        } else {
            let next_prime = if self.primes.len() == 0 {
                2.into()
            } else if self.primes.len() == 1 {
                3.into()
            } else {
                let mut candidate = *self.primes.last().unwrap();
                loop {
                    if candidate % 6.into() == 1.into() {
                        candidate = candidate + 4.into();
                    } else {
                        candidate = candidate + 2.into();
                    }

                    if Self::prime_test_against_vec(candidate, &self.primes) {
                        break;
                    }
                }
                candidate
            };

            self.primes.push(next_prime);
            next_prime
        };

        self.iter_index += 1;
        Some(result)
    }
}
