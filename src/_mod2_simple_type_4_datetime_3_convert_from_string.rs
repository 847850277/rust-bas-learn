use chrono::prelude::*;
pub(crate) fn test() {

    //the first method
    let string_dt1 = "1945-05-09 01:00";
    let victory_dt1 = Local.datetime_from_str(
        string_dt1, "%Y-%m-%d %H:%M");
    println!("{:?}", victory_dt1);

    //the second method
    let string_dt2 = "1945-05-09 01:00:00-03:00";
    let victory_dt2 = DateTime::parse_from_str(
        string_dt2, "%Y-%m-%d %H:%M:%S %z");
    println!("{:?}", victory_dt2);

}