use chrono::prelude::*;

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";
const DATE_FORMAT: &str = "%A %B %-d, %Y %H:%M:%S %Z";

// NOTE: -08:00 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
const BIRTHDATE: &str = "1977-04-05 11:58:00 -08:00";

fn main() {
    let now = Local::now();
    let birthdate = DateTime::parse_from_str(BIRTHDATE, PARSE_FORMAT).unwrap();
    let local_birthdate = birthdate.with_timezone(&now.timezone());
    let duration = now - local_birthdate;

    let weeks = duration.num_weeks();
    let days = duration.num_days() - (duration.num_weeks() * 7);
    let hours = duration.num_hours() - (duration.num_days() * 24);
    let minutes = duration.num_minutes() - (duration.num_hours() * 60);

    println!("Born on {}", birthdate.format(DATE_FORMAT));
    println!("Current time is {}\n", now.format(DATE_FORMAT));

    println!(
        "Alive for {} weeks, {} days, {} hours, and {} minutes",
        weeks, days, hours, minutes
    );
}
