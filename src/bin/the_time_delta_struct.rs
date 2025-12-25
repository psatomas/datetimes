use chrono::TimeDelta;

fn main() {
    let five_seconds = TimeDelta::new(5, 0);
    println!("{five_seconds:?}"); // Some(TimeDelta { secs: 5, nanos: 0 })

    let _invalid = TimeDelta::new(5, 1_000_000_000);

    let negative_five_seconds = TimeDelta::new(-5, 0).unwrap();
    println!("{negative_five_seconds:?}");

    let five_minutes = TimeDelta::minutes(5);
    println!("{five_minutes:?}");

    let negative_five_numbers = TimeDelta::minutes(-5);
    println!("{negative_five_numbers:?}");

    let five_hours = TimeDelta::hours(5);
    println!("{five_hours:?}");

    let five_days = TimeDelta::days(5);
    println!("{five_days:?}");

    let five_weeks = TimeDelta::weeks(5);
    println!("{five_weeks:?}");

    let total_duration = five_weeks + five_days + five_hours + five_minutes;

    println!(
        "{} weeks, {} days, {} hours, {} minutes",
        total_duration.num_weeks(),
        total_duration.num_days(),
        total_duration.num_hours(),
        total_duration.num_minutes()
    );
}