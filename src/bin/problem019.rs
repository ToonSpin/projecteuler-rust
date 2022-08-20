// How many Sundays fell on the first of the month during the twentieth century
// (1 Jan 1901 to 31 Dec 2000)?

fn is_leap(year: u32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        year % 4 == 0
    }
}

fn length_of_month(month: u32, year: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => if is_leap(year) { 29 } else { 28 },
        _ => 31,
    }
}

fn main() {
    let mut month = 1;
    let mut year = 1901;
    let mut day = 1; // January 1st 1901 was a Tuesday
    let mut sunday_count = 0;

    loop {
        if year > 2000 {
            break;
        }
        if day == 6 {
            sunday_count += 1;
        }
        day += length_of_month(month, year);
        day %= 7;
        month += 1;
        if month > 12 {
            year += 1;
            month = 1;
        }
    }
    println!("The number of Sundays in the twentieth century is: {}", sunday_count);
}
