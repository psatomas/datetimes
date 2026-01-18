/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/

use chrono::prelude::*;

fn main() {
    let event_data: Vec<(&'static str, &'static str)> = vec![
        (
            "2025**04**19 16:00:00 -04:00",
            "Started Rust study session",
        ),
        ("2025**04**19 18:30:00 -04:00", "Went to bed"),
        ("ERR", "ERR"),
        ("2025**04**19 18:30:00 -04:00", "Resumed Rust study"),
    ];

    let format = "%Y**%m**%d !! %H:%M:%S %z";

    let events: Vec<_> = event_data
        .into_iter()
        .filter_map(|(timestamp, message)| {
            let parsed_datetime = DateTime::parse_from_str(timestamp, format);
            match parsed_datetime {
                Ok(datetime) => Some((datetime.with_timezone(&Utc), message)),
                Err(_) => None,
            }
        })
        .collect();

    let mut previous_event: Option<DateTime<Utc>> = None;

    for (utc_datetime, message) in events {
        let display_time = utc_datetime.format("%Y-%m-%d %H:%M:%S UTC");
        println!("Event time: {display_time}");
        println!("Event message: {message}");
    }    
}