pub(crate) fn test() {
    let number = 42;
    match number {
        1 => print!("one"),
        x if x > 10 => print!("over 10"),
        _ => print!("other value"),
    }
}
