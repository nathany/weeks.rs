use weeks::Duration;

#[test]
fn duration() {
    let d = chrono::Duration::weeks(107)
        + chrono::Duration::days(5)
        + chrono::Duration::hours(13)
        + chrono::Duration::minutes(38);
    let duration = Duration::new(d);

    assert_eq!(duration.weeks, 107);
    assert_eq!(duration.days, 5);
    assert_eq!(duration.hours, 13);
    assert_eq!(duration.minutes, 38);

    assert_eq!(
        format!("{}", duration),
        "107 weeks, 5 days, 13 hours, and 38 minutes"
    )
}
