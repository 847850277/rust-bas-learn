#[allow(dead_code)]
enum Season {
    Summer,
    Fall,
    Winter,
    Spring,
}
pub(crate) fn test() {
    let winter = Season::Winter;
    let base_value = winter as u32;
    //baseValue is 2
    println!("base_value is {base_value}");
}
