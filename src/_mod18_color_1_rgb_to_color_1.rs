pub(crate) fn test() {
    let red = 51;
    let green = 255;
    let blue = 51;
    let alpha = 128;
    let mut c_green =
        "#".to_owned() + &format!("{:x}", red) + &format!("{:x}", green) + &format!("{:x}", blue);
    println!("c_green is {c_green}");
    //with transparency
    c_green = c_green.to_owned() + &format!("{:x}", alpha);
    println!("c_green is {c_green}");
}
