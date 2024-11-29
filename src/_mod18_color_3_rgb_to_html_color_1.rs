pub(crate) fn test() {
    let red = 51;
    let green = 255;
    let blue = 51;
    let alpha = 128;
    let mut html_color = format!("#{:X}{:X}{:X}", red, green, blue);
    println!("style=\"color: {html_color}\"");
    //with transparency
    html_color = format!("{html_color}{:X}", alpha);
    println!("html_color is {html_color}");
}
