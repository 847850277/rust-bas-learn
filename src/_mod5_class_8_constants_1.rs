// Constants begin with an uppercase letter
struct Calendar;
impl Calendar {
    const MONTHS: i8 = 12;
}
pub(crate) fn test() {
    let months = Calendar::MONTHS;
    //months is 12
    println!("months is {months}");
}
