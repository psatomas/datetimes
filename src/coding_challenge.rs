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
        (
            "2025**04**19 18:30:00 -04:00",
            "Ended Rust study session",
        ),
        ("ERR", "ERR"),
    ];

    let format = "%Y**%m**%d %H:%M:%S %z";

    for (timestamp, description) in event_data {
        match DateTime::parse_from_str(timestamp, format) {
            Ok(dt) => {
                println!("{} → {}", dt, description);
            }
            Err(_) => {
                println!("Invalid timestamp → {}", description);
            }
        }
    }
}
