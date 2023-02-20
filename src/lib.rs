use chrono::format::ParseResult;
use chrono::prelude::*;
use std::fmt;

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %#z";
const DATE_FORMAT: &str = "%A %B %-d, %Y %-I:%M %p %Z";

pub fn now() -> DateTime<Local> {
    return Local::now();
}

pub fn parse_date_time(s: &str) -> ParseResult<DateTime<FixedOffset>> {
    return DateTime::parse_from_str(s, PARSE_FORMAT);
}

pub fn format_date_time(dt: DateTime<FixedOffset>) -> String {
    return format!("{}", dt.format(DATE_FORMAT));
}

pub fn format_local_date_time(dt: DateTime<Local>) -> String {
    return format!("{}", dt.format(DATE_FORMAT));
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub birthdate: DateTime<FixedOffset>,
    pub pronoun: Pronoun,
}

#[derive(Debug)]
pub enum Pronoun {
    HeHim,
    SheHer,
}

impl Person {
    pub fn new(name: &str, birthdate: DateTime<FixedOffset>, pronoun: Pronoun) -> Person {
        Person {
            name: name.to_string(),
            birthdate: birthdate,
            pronoun: pronoun,
        }
    }

    pub fn age(&self, now: DateTime<Local>) -> String {
        let local_birthdate = self.birthdate.with_timezone(&now.timezone());
        let duration = Duration::new(now - local_birthdate);
        return format!("{}", duration);
    }
}

#[derive(Debug)]
pub struct Duration {
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} weeks, {} days, {} hours, and {} minutes",
            self.weeks, self.days, self.hours, self.minutes
        )
    }
}

impl Duration {
    pub fn new(duration: chrono::Duration) -> Duration {
        return Duration {
            weeks: duration.num_weeks(),
            days: duration.num_days() - (duration.num_weeks() * 7),
            hours: duration.num_hours() - (duration.num_days() * 24),
            minutes: duration.num_minutes() - (duration.num_hours() * 60),
        };
    }
}
