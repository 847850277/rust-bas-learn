use regex::Captures;
use regex::Regex;
pub(crate) fn test() {
    //"world" in hexadecimal format
    let data = "x77x6Fx72x6Cx64";
    let pattern = r"x([0-9A-F]{2})";
    let re = Regex::new(pattern).unwrap();
    // function
    let lambda = |caps: &Captures| {
        let code: u32 = u32::from_str_radix(&caps[1], 16).unwrap();
        let char_v = char::from_u32(code).unwrap();
        return format!("{char_v}");
    };
    let result = re.replace_all(data, lambda);
    //result is "world"
    println!("result is '{result}'");
}
