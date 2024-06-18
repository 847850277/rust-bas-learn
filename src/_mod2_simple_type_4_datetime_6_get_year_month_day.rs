use chrono::prelude::*;

pub(crate) fn test() {

    let now = chrono::offset::Local::now();
    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();
    let weekday = now.weekday();

    println!("year is {year}");
    println!("month is {month}");
    println!("day is {day}");
    println!("hour is {hour}");
    println!("minute is {minute}");
    println!("second is {second}");
    println!("weekday is '{weekday}'");

}