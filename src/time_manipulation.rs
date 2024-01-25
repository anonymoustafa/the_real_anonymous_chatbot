extern crate chrono;
use chrono::prelude::Utc;
fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let local: DateTime<Local> = Local::now();
    println!("{}", local);
}

