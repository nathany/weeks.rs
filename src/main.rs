// Calculate my age in weeks
// Inspired by Four Thousand Weeks by Oliver Burkeman.
use chrono::prelude::*;

const NAME: &str = "Nathan";
const PRONOUN: &str = "He";
// NOTE: -08:00 is PST. Daylight Saving Time started in B.C. on Sunday, April 24, 1977.
const BIRTHDATE: &str = "1977-04-05 11:58 -08:00";

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %:z";
const TIME_FORMAT: &str = "%A, %B %-d, %Y at %-I:%M %p (%:z)";

fn main() {
    let now = now();
    let birthdate = parse_date_time(BIRTHDATE);
    let (weeks, days, hours, minutes) = age(birthdate, now);

    println!("The current time is {}.\n", now.format(TIME_FORMAT));
    println!("{} was born on {}.", NAME, birthdate.format(TIME_FORMAT));
    println!(
        "{} has been alive for {} weeks, {} days, {} hours and {} minutes.",
        PRONOUN, weeks, days, hours, minutes
    );
}

fn now() -> DateTime<FixedOffset> {
    Local::now().fixed_offset()
}

fn parse_date_time(s: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(s, PARSE_FORMAT).unwrap()
}

fn age(birthdate: DateTime<FixedOffset>, now: DateTime<FixedOffset>) -> (i64, i64, i64, i64) {
    let local_birthdate = birthdate.with_timezone(&now.timezone());
    let duration = now - local_birthdate;

    let weeks = duration.num_weeks();
    let days = duration.num_days() - (duration.num_weeks() * 7);
    let hours = duration.num_hours() - (duration.num_days() * 24);
    let minutes = duration.num_minutes() - (duration.num_hours() * 60);

    (weeks, days, hours, minutes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_time() {
        let s = "2024-09-18 19:27 -06:00";
        let dt = parse_date_time(s);
        assert_eq!(dt.format(PARSE_FORMAT).to_string(), s);
    }

    #[test]
    fn test_age() {
        // -06:00 is MDT.
        let now = parse_date_time("2024-09-18 19:27 -06:00");
        let birthdate = parse_date_time("1977-04-05 11:58 -08:00");
        let (weeks, days, hours, minutes) = age(birthdate, now);

        assert_eq!(weeks, 2476);
        assert_eq!(days, 1);
        assert_eq!(hours, 5);
        assert_eq!(minutes, 29);
    }
}
