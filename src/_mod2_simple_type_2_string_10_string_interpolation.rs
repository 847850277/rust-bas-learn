pub(crate) fn test() {
    let font_size = 14;
    let font_famile = "Arial";
    let style = format!(
        "font-size: {};font-family: {}",
        font_size, font_famile);
    //style is "font-size: 14;font-family: Arial"

    println!(r#"style is "{style}""#);

}