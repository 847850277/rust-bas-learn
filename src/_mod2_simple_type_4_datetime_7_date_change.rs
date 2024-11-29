use chrono;
use chrono::{Duration, NaiveDate};
pub(crate) fn test() {
    let now = chrono::offset::Utc::now();
    let yesterday = now - Duration::days(1);
    let tomorrow = now + Duration::days(1);

    println!("yesterday is {yesterday}");
    println!("tomorrow is {tomorrow}");
}
