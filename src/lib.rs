use std::fmt;

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
