use regex::Regex;
pub(crate) fn test() {
    let data1 = "aaab";
    let data2 = "aaaba";
    let pattern = r"^a+b$";
    let re = Regex::new(pattern).unwrap();
    let b1 = re.find(data1).is_some();
    //b1 is true
    let b2 = re.find(data2).is_some();
    //b2 is false
    println!("b1 is {b1}");
    println!("b2 is {b2}");
}
