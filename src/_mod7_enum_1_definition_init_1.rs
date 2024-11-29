enum Season {
    Summer,
    Fall,
    Winter,
    Spring,
}

pub(crate) fn test() {
    let summer = Season::Summer;
    let winter = Season::Winter;
    println!("summer is {:#?}", summer as u32);
    println!("winter is {:#?}", winter as u32);
}
