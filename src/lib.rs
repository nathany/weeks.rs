use chrono::Duration;
use std::fmt;

#[derive(Debug)]
pub struct Weeks {
    pub weeks: i64,
    pub days: i64,
    pub hours: i64,
    pub minutes: i64,
}

impl fmt::Display for Weeks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} weeks, {} days, {} hours, and {} minutes",
            self.weeks, self.days, self.hours, self.minutes
        )
    }
}

impl Weeks {
    pub fn new(duration: Duration) -> Weeks {
        return Weeks {
            weeks: duration.num_weeks(),
            days: duration.num_days() - (duration.num_weeks() * 7),
            hours: duration.num_hours() - (duration.num_days() * 24),
            minutes: duration.num_minutes() - (duration.num_hours() * 60),
        };
    }
}
