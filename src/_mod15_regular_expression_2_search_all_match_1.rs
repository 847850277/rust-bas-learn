use regex::Regex;
pub(crate) fn test() {

    let data = "Pi = 3.14, exponent = 2.718";
    let pattern = r"\d+\.\d+";
    let re = Regex::new(pattern).unwrap();
    let nums: Vec<&str> = re.find_iter(data)
        .map(|x| x.as_str()).collect();
    //nums is ["3.14", "2.718"]
    println!("nums is {:?}", nums);

}