/// Calculate my age in weeks
/// Inspired by Four Thousand Weeks by Oliver Burkeman.
use weeks::{age, format_birthdate, format_local, now, parse_date_time};

const NAME: &str = "Nathan";
const PRONOUN: &str = "He";
// NOTE: -08 is PST. Daylight Saving Time started in B.C. on Sunday, April 24, 1977.
const BIRTH_TIME: &str = "1977-04-05 11:58 -08";

fn main() {
    let now = now();

    let birthdate = parse_date_time(BIRTH_TIME).unwrap();

    println!("The current time is {}.\n", format_local(now));

    println!("{} was born on {}.", NAME, format_birthdate(birthdate));
    println!("{} has been alive for {}.", PRONOUN, age(birthdate, now));
}
