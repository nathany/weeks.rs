use chrono::Duration;
use weeks::Weeks;

#[test]
fn duration() {
    let d = Duration::weeks(107) + Duration::days(5) + Duration::hours(13) + Duration::minutes(38);
    let duration = Weeks::new(d);

    assert_eq!(duration.weeks, 107);
    assert_eq!(duration.days, 5);
    assert_eq!(duration.hours, 13);
    assert_eq!(duration.minutes, 38);

    assert_eq!(
        format!("{}", duration),
        "107 weeks, 5 days, 13 hours, and 38 minutes"
    )
}
