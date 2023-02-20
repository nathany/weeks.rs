use weeks::Age;

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
        "107 weeks, 5 days, 13 hours, and 38 minutes"
    )
}
