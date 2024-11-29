pub(crate) fn test() {
    let orange: u32 = 0xFFC80080;
    let red = (orange >> 24) & 0xFF;
    let green = (orange >> 16) & 0xFF;
    let blue = (orange >> 8) & 0xFF;
    let alpha = orange & 0xFF;
    println!("red is {red}");
    println!("green is {green}");
    println!("blue is {blue}");
    println!("alpha is {alpha}");
}
