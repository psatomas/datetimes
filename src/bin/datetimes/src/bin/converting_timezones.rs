use chrono::prelude::*;
use chrono_tz::America::Los_Angeles;

fn main() {
    let local_time = Local::now();
    let utc_time = local_time.with_timezone(&Utc);

    println!("{}", local_time);
    println!("{}", utc_time);

    println!("{}", utc_time.with_timezone(&Local));

    let la_time = local_time.with_timezone(&Los_Angeles);

    println!("{}", local_time);
    println!("{}", la_time);
}