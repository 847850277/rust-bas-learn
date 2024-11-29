pub(crate) fn test() {
    let start_string = "3, 2, 1, go!";
    let start_string = start_string
        .replace("1", "one")
        .replace("2", "two")
        .replace("3", "three");
    // start_string = "three, two, one, go!"

    print!("start_string is \"{start_string}\"");
}
