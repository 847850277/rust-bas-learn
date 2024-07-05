pub(crate) fn test() {
    let orange: u32 = 0xFFC80080;
    let x = (orange >> 8) & 0xFFFFFF;
    let html_color = format!("{:X}", x);
    //html_color is #33ff33
    println!(r#"style="color: #{html_color}""#);
    //with transparency
    let html_color = format!("{:X}", orange);
    //html_color is #33ff33
    println!(r#"style="color: #{html_color}""#);

}