#[allow(dead_code)]
enum Season {
    Summer = 1,
    Fall,
    Winter,
    Spring,
}
pub(crate) fn test() {
    let winter = Season::Winter;
    println!("winter is {:#?}", winter as u32);
}
