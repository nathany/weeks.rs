/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use chrono::prelude::*;
use weeks::Duration;

// NOTE: -08 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
const BIRTHDATE: &str = "1977-04-05 11:58 -08";

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %#z";
const DATE_FORMAT: &str = "%A %B %-d, %Y %-I:%M %p %Z";

fn main() {
    let now = Local::now();
    let birthdate = DateTime::parse_from_str(BIRTHDATE, PARSE_FORMAT).unwrap();
    let local_birthdate = birthdate.with_timezone(&now.timezone());
    let duration = Duration::new(now - local_birthdate);

    println!("Born on {}", birthdate.format(DATE_FORMAT));
    println!("Current time is {}\n", now.format(DATE_FORMAT));
    println!("Alive for {}", duration);
}
