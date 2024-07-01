use regex::Regex;
pub(crate) fn test() {

    let data = "Pi = 3.14, exponent = 2.718";
    let pattern = r"\d+\.\d+";
    let re = Regex::new(pattern).unwrap();
    let data = re.replace_all(data, "<f>$1</f>");
    println!("data is {data}");

}