fn add3_and_print(value: i32) {
    print!("{}", value + 3);
}

pub(crate) fn test() {
    add3_and_print(5);
}