/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use weeks::{format_date_time, format_local_date_time, now, parse_date_time, Person, Pronoun};

// NOTE: -08 is PST. Daylight saving time started in B.C. on Sunday, April 24, 1977.
const BIRTHDATE: &str = "1977-04-05 11:58 -08";

fn main() {
    let now = now();
    let birthdate = parse_date_time(BIRTHDATE).unwrap();
    let person = Person::new("Nathan", birthdate, Pronoun::HeHim);

    println!("Current time is {}\n", format_local_date_time(now));

    println!("Born on {}", format_date_time(birthdate));
    println!("Alive for {}", person.age(now));

    println!("{:?}", person);
}
