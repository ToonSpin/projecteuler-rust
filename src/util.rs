pub trait UnsignedInt:
    From<u8>
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + Copy
{
}

impl UnsignedInt for u8 {}
impl UnsignedInt for u16 {}
impl UnsignedInt for u32 {}
impl UnsignedInt for u64 {}
impl UnsignedInt for u128 {}

pub trait Int:
    From<u8>
    + PartialOrd
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
    + Copy
{
}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}

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

pub fn get_digits<T: UnsignedInt>(mut n: T) -> Vec<T> {
    let mut digits = Vec::new();
    while n > 0.into() {
        digits.push(n % 10.into());
        n = n / 10.into();
    }
    digits
}

pub fn is_palindromic<T: UnsignedInt>(n: T) -> bool {
    let digits = get_digits(n);
    let max = digits.len() / 2 + 1;
    for i in 0..max {
        if digits[i] != digits[digits.len() - 1 - i] {
            return false;
        }
    }
    true
}
