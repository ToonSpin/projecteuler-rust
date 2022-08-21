use std::fmt::Debug;
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

    fn carry_digits<T>(digits: &[T]) -> Vec<u8>
    where
        T: UnsignedInt + TryInto<u8>,
        <T as TryInto<u8>>::Error: Debug
    {
        let mut new_digits: Vec<u8> = Vec::new();

        // make the digits carry over for all digits > 9
        let mut carry: T = (0).into();
        for digit in digits.iter() {
            let total: T = *digit + carry;
            let new_digit: T = total % (10.into());
            new_digits.push(new_digit.try_into().unwrap());
            carry = (total - new_digit) / (10.into());
        }

        // add any "leftover" carry information
        while carry > (0).into() {
            let new_digit: T = carry % (10.into());
            new_digits.push(new_digit.try_into().unwrap());
            carry = carry - new_digit;
            carry = carry / 10.into();
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
    T: UnsignedInt,
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

impl std::ops::Add<&DigitalNumber> for DigitalNumber {
    type Output = Self;

    fn add(self, rhs: &DigitalNumber) -> Self {
        let mut new_digits = Vec::new();
        let lhs_digits;
        let rhs_digits;

        // make sure lhs_digits has as least as many elements as rhs_digits
        if self.digits.len() < rhs.digits.len() {
            lhs_digits = &rhs.digits;
            rhs_digits = &self.digits;
        } else {
            lhs_digits = &self.digits;
            rhs_digits = &rhs.digits;
        }

        // simply add the numbers
        for (i, digit) in lhs_digits.iter().enumerate() {
            let rhs_digit: u8 = match rhs_digits.get(i) {
                Some(&d) => d,
                None => 0,
            };
            new_digits.push(digit + rhs_digit);
        }

        new_digits = Self::carry_digits(&new_digits);

        DigitalNumber { digits: new_digits }
    }
}

impl std::ops::Add<DigitalNumber> for DigitalNumber {
    type Output = Self;
    fn add(self, rhs: DigitalNumber) -> Self {
        return self + &rhs;
    }
}

impl<T> std::ops::Mul<T> for DigitalNumber
where
    T: UnsignedInt + TryInto<u8>,
    <T as TryInto<u8>>::Error: Debug
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut multiplied_digits: Vec<T> = Vec::new();
        for digit in self.digits.iter() {
            multiplied_digits.push(T::from(*digit) * rhs);
        }
        DigitalNumber {
            digits: Self::carry_digits(&multiplied_digits)
        }
    }
}
