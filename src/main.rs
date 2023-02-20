use chrono::prelude::*;

const INPUT_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

// NOTE: -08:00 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
const BIRTH_DATE: &str = "1977-04-05 11:58:00 -08:00";

fn main() {
    let now = Local::now();
    let birth_date = DateTime::parse_from_str(BIRTH_DATE, INPUT_FORMAT).unwrap();
    let local_birth_date = birth_date.with_timezone(&now.timezone());
    let duration = now - local_birth_date;

    let weeks = duration.num_weeks();
    let days = duration.num_days() - (duration.num_weeks() * 7);
    let hours = duration.num_hours() - (duration.num_days() * 24);
    let minutes = duration.num_minutes() - (duration.num_hours() * 60);

    println!("Born on {}", birth_date);
    println!("Current time is {}\n", now);

    println!(
        "Alive for {} weeks, {} days, {} hours, and {} minutes",
        weeks, days, hours, minutes
    );
}
