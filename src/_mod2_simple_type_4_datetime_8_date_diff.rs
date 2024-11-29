use chrono::*;
pub(crate) fn test() {
    let victory_date = Utc.ymd(1945, 5, 9).and_hms(0, 0, 0);
    let now = chrono::offset::Utc::now();
    let delta: Duration = now - victory_date;
    let days = delta.num_days();
    let minutes = delta.num_minutes();
    let seconds = delta.num_seconds();

    println!("days is {days}");
    println!("minutes is {minutes}");
    println!("seconds is {seconds}");
}
