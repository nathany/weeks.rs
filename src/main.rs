/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use chrono::prelude::*;

// NOTE: -08 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
const BIRTHDATE: &str = "1977-04-05 11:58 -08";

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %#z";
const DATE_FORMAT: &str = "%A %B %-d, %Y %-I:%M %p %Z";

#[derive(Debug)]
struct Weeks {
    weeks: i64,
    days: i64,
    hours: i64,
    minutes: i64,
}

fn calculate(now: DateTime<Local>, birthdate: DateTime<FixedOffset>) -> Weeks {
    let local_birthdate = birthdate.with_timezone(&now.timezone());
    let duration = now - local_birthdate;

    return Weeks {
        weeks: duration.num_weeks(),
        days: duration.num_days() - (duration.num_weeks() * 7),
        hours: duration.num_hours() - (duration.num_days() * 24),
        minutes: duration.num_minutes() - (duration.num_hours() * 60),
    };
}

fn main() {
    let now = Local::now();
    let birthdate = DateTime::parse_from_str(BIRTHDATE, PARSE_FORMAT).unwrap();
    let duration = calculate(now, birthdate);

    println!("Born on {}", birthdate.format(DATE_FORMAT));
    println!("Current time is {}\n", now.format(DATE_FORMAT));

    println!(
        "Alive for {} weeks, {} days, {} hours, and {} minutes",
        duration.weeks, duration.days, duration.hours, duration.minutes
    );
}
