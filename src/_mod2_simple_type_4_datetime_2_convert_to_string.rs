use chrono;

pub(crate) fn test() {

    let now = chrono::Local::now();

    let short_style_ru = now.format("%d.%m.%Y %H:%M");
    //shortStyleEn is "16.05.2022 00:51"
    println!("short_style_ru is '{short_style_ru}'");

    //let h = now.hour() % 12;
    let short_style_en = now.format("%m/%d/%Y %H:%M");
    //shortStyleEn is "05/16/2022 00:51"
    println!("short_style_en is '{short_style_en}'");

    let custom_style = now.format("%Y-%m-%d");
    //custom_style is "2015-06-22"
    println!("custom_style is '{custom_style}'");

}