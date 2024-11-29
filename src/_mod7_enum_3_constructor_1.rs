#[allow(dead_code)]
enum Season {
    Summer,
    Fall,
    Winter,
    Spring,
    None,
}
impl Season {
    fn new(name: &str) -> Season {
        match name {
            "summer" => return Season::Summer,
            "fall" => return Season::Fall,
            "winter" => return Season::Winter,
            "spring" => return Season::Spring,
            _ => return Season::None,
        }
    }
}

pub(crate) fn test() {
    let winter = Season::new("winter");
    //winter is Season.Winter
    println!("winter is {:#?}", winter as u32);
}
