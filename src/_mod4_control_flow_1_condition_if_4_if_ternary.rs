pub(crate) fn test() {
    let n = -42;
    let classify = if n > 0 { "positive" } else { "negative" };
    //classify is "negative"
    println!("classify is '{classify}'");
}
