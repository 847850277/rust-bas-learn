pub(crate) fn test() {
    let multiline_string = "
    1. First line
    2. Second \
    line
        3. Third line
    4. Fourth \"line\"";

    print!("{multiline_string}");

//prints:
//
//    1. First line
//    2. Second line
//        3. Third line
//    4. Fourth "line"

}