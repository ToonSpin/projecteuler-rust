use crate::util::UnsignedInt;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct DigitalNumber {
    digits: Vec<u8>,
}

impl Default for DigitalNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalNumber {
    pub fn new() -> Self {
        Self { digits: Vec::new() }
    }

    pub fn get_digits(&self) -> Vec<u8> {
        self.digits.clone()
    }

    fn carry_digits(digits: &[u8]) -> Vec<u8> {
        let mut new_digits = Vec::new();

        // make the digits carry over for all digits > 9
        let mut carry = 0;
        for digit in digits.iter() {
            let total = *digit + carry;
            let new_digit = total % 10;
            new_digits.push(new_digit);
            carry = (total - new_digit) / 10;
        }

        // add any "leftover" carry information
        while carry > 0 {
            let new_digit = carry % 10;
            new_digits.push(new_digit);
            carry -= new_digit;
            carry /= 10;
        }

        new_digits
    }
}

impl FromStr for DigitalNumber {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<DigitalNumber, ParseIntError> {
        let mut digits = VecDeque::new();
        for digit in s.as_bytes() {
            let digit_str = &[*digit];
            let new_digit = std::str::from_utf8(digit_str).unwrap();
            let new_digit = new_digit.parse()?;
            digits.push_front(new_digit);
        }
        Ok(Self { digits: Vec::from(digits) })
    }
}

impl<T> From<T> for DigitalNumber
where
    T: UnsignedInt + std::ops::Sub<Output = T>,
    u8: From<T>
{
    fn from(mut n: T) -> DigitalNumber {
        let mut digits: Vec<u8> = Vec::new();
        while n > T::from(0) {
            let new_digit: T = n % T::from(10u8);
            digits.push(new_digit.into());
            n = (n - new_digit) / T::from(10u8);
        }
        DigitalNumber { digits }
    }
}

impl std::ops::AddAssign<DigitalNumber> for DigitalNumber {
    fn add_assign(&mut self, rhs: DigitalNumber) {
        let sum = self.clone() + rhs;
        self.digits = sum.digits;
    }
}

impl std::ops::Add<DigitalNumber> for DigitalNumber {
    type Output = Self;

    fn add(self, rhs: DigitalNumber) -> Self {
        let mut new_digits = Vec::new();

        // make sure self has as least as many elements as rhs
        if self.digits.len() < rhs.digits.len() {
            return rhs + self;
        }

        // simply add the numbers
        for (i, digit) in self.digits.iter().enumerate() {
            let rhs_digit: u8 = match rhs.digits.get(i) {
                Some(&d) => d,
                None => 0,
            };
            new_digits.push(digit + rhs_digit);
        }

        new_digits = Self::carry_digits(&new_digits);

        DigitalNumber { digits: new_digits }
    }
}
