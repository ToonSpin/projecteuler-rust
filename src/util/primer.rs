use crate::util::UnsignedInt;

pub struct Primer<T: UnsignedInt> {
    primes: Vec<T>,
    iter_index: usize,
}

impl<T: UnsignedInt> Default for Primer<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: UnsignedInt> Primer<T> {
    pub fn new() -> Primer<T> {
        Primer {
            primes: Vec::new(),
            iter_index: 0,
        }
    }

    fn prime_test_against_slice(n: T, primes: &[T]) -> bool {
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
            if Self::prime_test_against_slice(next_prime, &primes) {
                primes.push(next_prime);
            }
            next_prime = next_prime + 2.into();

            if Self::prime_test_against_slice(next_prime, &primes) {
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
        self.primes
    }

    pub fn reset(&mut self) {
        self.iter_index = 0;
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
            let next_prime = if self.primes.is_empty() {
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

                    if Self::prime_test_against_slice(candidate, &self.primes) {
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

pub fn prime_factorization<T: UnsignedInt>(mut n: T, primer: &mut Primer<T>) -> Vec<(T, usize)> {
    let mut v = Vec::new();
    primer.reset();
    for p in primer {
        if n <= 1.into() {
            break;
        }
        let mut p_count = 0;
        while n % p == 0.into() {
            n = n / p;
            p_count += 1;
        }
        if p_count > 0 {
            v.push((p, p_count));
        }
    }
    v
}
