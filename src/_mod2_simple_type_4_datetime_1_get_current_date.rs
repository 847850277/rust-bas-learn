use chrono;
pub(crate) fn test() {

    let now_local = chrono::offset::Local::now();
    let now_utc = chrono::offset::Utc::now();
    println!("{now_local}");
    println!("{now_utc}");

}