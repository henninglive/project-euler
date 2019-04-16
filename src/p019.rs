//! **Problem 19**: Counting Sundays
//!
//! You are given the following information, but you may prefer to do some research for yourself.
//!
//! * 1 Jan 1900 was a Monday.
//! * Thirty days has September,
//!   April, June and November.
//!   All the rest have thirty-one,
//!   Saving February alone,
//!   Which has twenty-eight, rain or shine.
//!   And on leap years, twenty-nine.
//! * A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//!
//! How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
//!
//! [Problem 19 on projecteuler.net](https://projecteuler.net/problem=19)
//!

fn is_leap_year(year: usize) -> bool {
    ((year % 4 == 0) && (year % 100 != 0)) || year % 400 == 0
}

fn days_in_month(month: usize, year: usize) -> usize {
    match month {
        0 =>  31,
        1 =>  if is_leap_year(year) { 29 } else { 28 },
        2 =>  31,
        3 =>  30,
        4 =>  31,
        5 =>  30,
        6 =>  31,
        7 =>  31,
        8 =>  30,
        9 =>  31,
        10 => 30,
        11 => 31,
        _ => panic!(),
    }
}

/// Calculate solution to Problem 19
pub fn solution() -> String {
    let mut year = 1900;
    let mut month = 0;
    let mut weekday = 0;
    let mut day = 0;
    let mut count = 0;
    
    while year < 2001 {
        if year > 1900 && weekday == 6 && day == 0 {
            count += 1;
        }
        
        weekday += 1;
        if weekday > 6 {
            weekday = 0;
        }
        
        day += 1;
        if day > days_in_month(month, year) {
            day = 0;
            month += 1;
            if month > 11 {
                month = 0;
                year += 1;
            }
        }
    }

    count.to_string()
}
