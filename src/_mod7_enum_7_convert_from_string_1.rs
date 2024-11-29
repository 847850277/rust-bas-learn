use std::str::FromStr;
use strum_macros::EnumString;
#[derive(EnumString)]
enum Season {
    Summer,
    Fall,
    Winter,
    Spring,
}

pub(crate) fn test() {
    let winter = Season::from_str("Winter").unwrap();
    //winter is Season.Winter
    println!("winter is {:#?}", winter as u32);
}
