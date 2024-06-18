use chrono::prelude::*;

pub(crate) fn test() {


    let year = 1945;
    let month = 5;
    let day = 9;
    let victory_date = Utc.ymd(year, month, day);

    println!("{victory_date}");

}