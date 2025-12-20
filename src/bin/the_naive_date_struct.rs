use chrono::NaiveDate;

fn main() {
    let birthday = "1991-04-12";
    let birthday = birthday
        .parse::<NaiveDate>()
        .expect("Unable to parse NaiveDate from string");

    println!("{birthday}");
}