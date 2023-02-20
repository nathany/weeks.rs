use chrono::prelude::*;

fn main() {
    let now = Local::now();
    // NOTE: -08:00 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
    let birthdate =
        DateTime::parse_from_str("1977-04-05 11:58:00 -08:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
    println!("Current time is {}", now);
    println!("Born on {:?}", birthdate);

    // Alive for 2393 weeks, 5 days, 3 hours, and 29 minutes.
}
