use chrono::prelude::*;

fn main() {
    let someday = "31-Oct-1995 18:07:54 -0600";
    let dt = DateTime::parse_from_str(someday, "%d-%b-%Y %H:%M:%S %z");
    println!("{dt:?}"); // Ok(1995-10-31T18:07:54-06:00)
}