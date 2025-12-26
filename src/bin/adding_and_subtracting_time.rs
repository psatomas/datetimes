use chrono::{NaiveDate, TimeDelta};
use std::ops::{Add, Sub};

fn main() {
    let birthday = NaiveDate::from_ymd_opt(1991, 4, 12).unwrap();
    println!("{}", birthday.add(TimeDelta::days(5)));

    println!("{}", birthday.add(TimeDelta::weeks(2) + TimeDelta::days(5)));

    println!("{}", birthday.sub(TimeDelta::weeks(3)));

    println!("{}", birthday + TimeDelta::days(5));

    println!("{}", birthday + (TimeDelta::weeks(2) + TimeDelta::days(5)));

    println!("{}", birthday - TimeDelta::weeks(3));
    println!("{}", birthday + TimeDelta::weeks(-3));

    // println!("{}", birthday.add(TimeDelta::days(100_000_000)));
}