/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use weeks::{format_local_date_time, now, parse_date_time, Person, Pronoun};

fn main() {
    let now = now();

    // NOTE: -08 is PST. Daylight Saving Time started in B.C. on Sunday, April 24, 1977.
    let birthdate = parse_date_time("1977-04-05 11:58 -08").unwrap();
    let person = Person::new("Nathan", Pronoun::HeHim, birthdate, "British Columbia");

    println!("Current time is {}\n", format_local_date_time(now));

    println!("{} was born on {}.", person.name, person.birth());
    println!(
        "{} has been alive for {}.",
        person.pronoun.subjective(true),
        person.age(now)
    );
}
