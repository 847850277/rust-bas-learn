use std::fmt;
#[allow(dead_code)]
#[derive(Debug)]
enum Season {
    Summer,
    Fall,
    Winter,
    Spring,
}
impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub(crate) fn test() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("summer is '{}'", summer.to_string());
    println!("winter is '{}'", winter.to_string());
}
