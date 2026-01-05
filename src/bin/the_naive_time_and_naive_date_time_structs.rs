use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

fn main() {
    let four_thirty_am = NaiveTime::from_hms_opt(4, 30, 0);
    println!("{:?}", four_thirty_am);

    let four_thirty_pm = NaiveTime::from_hms_opt(16, 30, 0);
    println!("{:?}", four_thirty_pm);

    let day = NaiveDate::from_ymd_opt(1969, 7, 20).unwrap();
    let time = NaiveTime::from_hms_opt(20, 17, 0).unwrap();
    let moon_landing = NaiveDateTime::new(day, time);
    println!("{moon_landing:?}");

    println!("{}", moon_landing + TimeDelta::days(1000));
    println!(
        "{}",
        moon_landing + TimeDelta::days(1000) + TimeDelta::minutes(45)
    );
}