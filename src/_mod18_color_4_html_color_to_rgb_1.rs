pub(crate) fn test() {
    let orange = "#FFC80080";
    if let Ok(x) = i64::from_str_radix(
        &orange[1..], 16) {
        let red = (x >> 24) & 0xFF;
        let green = (x >> 16) & 0xFF;
        let blue = (x >> 8) & 0xFF;
        let alpha = x & 0xFF;
        println!("red is {red}");
        println!("green is {green}");
        println!("blue is {blue}");
        println!("alpha is {alpha}");
    }

}