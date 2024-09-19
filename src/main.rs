/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use chrono::format::ParseResult;
use chrono::prelude::*;

const NAME: &str = "Nathan";
const PRONOUN: &str = "He";
// NOTE: -08 is PST. Daylight Saving Time started in B.C. on Sunday, April 24, 1977.
const BIRTH_TIME: &str = "1977-04-05 11:58 -08";

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %#z";
const TIME_FORMAT: &str = "%A, %B %-d, %Y at %-I:%M %p (%Z)";

fn main() {
    let now = Local::now().fixed_offset();
    let birthdate = parse_date_time(BIRTH_TIME).unwrap();

    println!("The current time is {}.\n", now.format(TIME_FORMAT));

    println!("{} was born on {}.", NAME, birthdate.format(TIME_FORMAT));
    let (weeks, days, hours, minutes) = age(birthdate, now);
    println!(
        "{} has been alive for {} weeks, {} days, {} hours and {} minutes.",
        PRONOUN, weeks, days, hours, minutes
    );
}

pub fn parse_date_time(s: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(s, PARSE_FORMAT)
}

pub fn age(birthdate: DateTime<FixedOffset>, now: DateTime<FixedOffset>) -> (i64, i64, i64, i64) {
    let local_birthdate = birthdate.with_timezone(&now.timezone());
    let duration = now - local_birthdate;

    let weeks = duration.num_weeks();
    let days = duration.num_days() - (duration.num_weeks() * 7);
    let hours = duration.num_hours() - (duration.num_days() * 24);
    let minutes = duration.num_minutes() - (duration.num_hours() * 60);

    (weeks, days, hours, minutes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_age() {
        // -06:00 is MDT.
        let now = make_time(2024, 9, 18, 19, 27, 0, -6 * 3600);
        let birthdate = parse_date_time("1977-04-05 11:58 -08").unwrap();
        let (weeks, days, hours, minutes) = age(birthdate, now.fixed_offset());

        assert_eq!(weeks, 2476, "expected 2476 weeks, got {}", weeks);
        assert_eq!(days, 1, "expected 1 days, got {}", days);
        assert_eq!(hours, 5, "expected 5 hours, got {}", hours);
        assert_eq!(minutes, 29, "expected 29 minutes, got {}", minutes);
    }

    fn make_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
        tz_offset_secs: i32,
    ) -> DateTime<FixedOffset> {
        FixedOffset::east_opt(tz_offset_secs)
            .unwrap()
            .from_local_datetime(
                &NaiveDate::from_ymd_opt(year, month, day)
                    .unwrap()
                    .and_hms_milli_opt(hour, min, sec, 0)
                    .unwrap(),
            )
            .unwrap()
    }
}
