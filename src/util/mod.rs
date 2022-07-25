pub mod maxpathsum;
pub mod primer;

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
