pub trait UnsignedInt:
    From<u8>
    + PartialOrd
    + std::ops::Add<Output = Self>
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
}

impl<T: UnsignedInt> Primer<T> {
    pub fn new() -> Primer<T> {
        Primer { primes: Vec::new() }
    }

    pub fn init_with_max_value(max_value: T) -> Primer<T> {
        let mut primes: Vec<T> = vec![2.into()];
        let mut next_prime: T = 3.into();

        while next_prime <= max_value {
            let mut not_prime = false;

            for p in primes.iter() {
                if next_prime % *p == 0.into() {
                    not_prime = true;
                    break;
                }
            }

            if !not_prime {
                primes.push(next_prime);
            }

            next_prime = next_prime + 2.into();
        }

        Primer { primes }
    }

    pub fn get_primes(self) -> Vec<T> {
        self.primes.clone()
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
