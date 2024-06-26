fn print5(mut data: &str) {
    if data.chars().count() > 5 {
        data = &data[0..5];
    }
    println!("{}", data);
}

pub(crate) fn test() {
    print5("1234567");
    //printed "12345"

}