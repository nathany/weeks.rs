use chrono::format::ParseResult;
use chrono::prelude::*;
use std::fmt;

const PARSE_FORMAT: &str = "%Y-%m-%d %H:%M %#z";
const DATE_FORMAT: &str = "%A, %B %-d, %Y";
const TIME_FORMAT: &str = "%-I:%M %p";
const TIMEZONE_FORMAT: &str = "%Z";

pub fn now() -> DateTime<Local> {
    Local::now()
}

pub fn parse_date_time(s: &str) -> ParseResult<DateTime<FixedOffset>> {
    DateTime::parse_from_str(s, PARSE_FORMAT)
}

pub fn format_local(dt: DateTime<Local>) -> String {
    format!(
        "{} at {} ({})",
        dt.format(DATE_FORMAT),
        dt.format(TIME_FORMAT),
        dt.format(TIMEZONE_FORMAT)
    )
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub pronoun: Pronoun,
    pub birthdate: DateTime<FixedOffset>,
}

#[derive(Debug)]
pub enum Pronoun {
    HeHim,
    SheHer,
}

#[derive(Debug)]
pub enum Case {
    Capitalize,
    Lowercase,
}

impl Person {
    pub fn new(name: &str, pronoun: Pronoun, birthdate: DateTime<FixedOffset>) -> Person {
        Person {
            name: name.to_string(),
            pronoun,
            birthdate,
        }
    }

    pub fn birth(&self) -> String {
        format!(
            "{} at {} ({})",
            self.birthdate.format(DATE_FORMAT),
            self.birthdate.format(TIME_FORMAT),
            self.birthdate.format(TIMEZONE_FORMAT)
        )
    }

    pub fn age(&self, now: DateTime<Local>) -> Age {
        let local_birthdate = self.birthdate.with_timezone(&now.timezone());
        Age::new(now - local_birthdate)
    }
}

impl Pronoun {
    pub fn subjective(&self, case: Case) -> String {
        match case {
            Case::Capitalize => match self {
                Pronoun::HeHim => "He".to_string(),
                Pronoun::SheHer => "She".to_string(),
            },
            Case::Lowercase => match self {
                Pronoun::HeHim => "he".to_string(),
                Pronoun::SheHer => "she".to_string(),
            },
        }
    }

    pub fn objective(&self) -> String {
        match self {
            Pronoun::HeHim => "him".to_string(),
            Pronoun::SheHer => "her".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Age {
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} weeks, {} days, {} hours and {} minutes",
            self.weeks, self.days, self.hours, self.minutes
        )
    }
}

impl Age {
    pub fn new(duration: chrono::Duration) -> Age {
        Age {
            weeks: duration.num_weeks(),
            days: duration.num_days() - (duration.num_weeks() * 7),
            hours: duration.num_hours() - (duration.num_days() * 24),
            minutes: duration.num_minutes() - (duration.num_hours() * 60),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn age() {
        let duration = chrono::Duration::weeks(107)
            + chrono::Duration::days(5)
            + chrono::Duration::hours(13)
            + chrono::Duration::minutes(38);
        let age = Age::new(duration);

        assert_eq!(age.weeks, 107);
        assert_eq!(age.days, 5);
        assert_eq!(age.hours, 13);
        assert_eq!(age.minutes, 38);

        assert_eq!(
            format!("{}", age),
            "107 weeks, 5 days, 13 hours and 38 minutes"
        )
    }
}
