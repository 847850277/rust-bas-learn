use regex::Regex;

pub(crate) fn test() {
    let data1 = "aaab";
    let data2 = "aaaba";
    let data3 = "bbba";
    let pattern = r"a+b";
    let re = Regex::new(pattern).unwrap();
    let b1 = re.find(data1).is_some();
    //b1 is true
    let b2 = re.find(data2).is_some();
    //b2 is true
    let b3 = re.find(data3).is_some();
    //b3 is false
    println!("b1 is {b1}");
    println!("b2 is {b2}");
    println!("b3 is {b3}");
}
