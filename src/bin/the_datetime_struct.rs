use chrono::prelude::*;

fn main() {
    let system_time = Local::now();
    let utc_time = Utc::now();

    println!("{}", system_time.date_naive());
    println!("{}", utc_time.date_naive());

    println!("{}", system_time.time());
    println!("{}", utc_time.time());

    println!("{}", system_time.year());
    println!("{}", utc_time.year());

    println!("{}", system_time.month());
    println!("{}", utc_time.month());

    println!("{}", system_time.day());
    println!("{}", utc_time.day());

    println!("{}", system_time.hour());
    println!("{}", utc_time.hour());

    println!("{}", system_time.minute());
    println!("{}", utc_time.minute());

    println!("{}", system_time.second());
    println!("{}", utc_time.second());

    println!("{}", system_time.offset());
    println!("{}", utc_time.offset());
}
