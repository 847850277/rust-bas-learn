use regex::Regex;
pub(crate) fn test() {
    let data = "Pi is equal to 3.14";
    let pattern = r"\d+\.\d+";
    let re = Regex::new(pattern).unwrap();
    let m = re.find(data);
    if m.is_some() {
        let pi_str = m.unwrap().as_str();
        let pi: f32 = pi_str.parse().unwrap();
        //pi is 3.14
        println!("pi is {pi}");
    }
}
