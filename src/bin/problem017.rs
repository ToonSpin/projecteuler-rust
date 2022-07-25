// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
// in words, how many letters would be used?

fn count_letters(number: u32) -> u32 {
    if number < 10 {
        match number {
            1 | 2 | 6 => 3,
            3 | 7 | 8 => 5,
            4 | 5 | 9 => 4,
            _ => 0,
        }
    } else if number < 100 {
        let tens = number / 10;
        let ones = number % 10;
        if tens == 1 {
            match ones {
                0 => 3,
                1 | 2 => 6,
                3 | 4 | 8 | 9 => 8,
                5 | 6 => 7,
                7 => 9,
                _ => unreachable!(),
            }
        } else {
            count_letters(ones) + match tens {
                4 | 5 | 6 => 5,
                2 | 3 | 8 | 9 => 6,
                7 => 7,
                _ => unreachable!(),
            }
        }
    } else {
        let hundreds = number / 100;
        let rest = number % 100;
        count_letters(hundreds) + 7 + if rest > 0 {
            3 + count_letters(rest)
        } else {
            0
        }
    }
}

fn main() {
    let mut sum = 0;
    for i in 1..=999 {
        sum += count_letters(i);
    }
    println!("The sum of the letters is: {}", sum + 11);
}
