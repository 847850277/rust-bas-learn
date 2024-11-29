use regex::Regex;
pub(crate) fn test() {
    let data = "AaaA\n\raaaA";
    let pattern = r"^a+$";
    let re = Regex::new(pattern).unwrap();
    if let Some(value) = re.find(data) {
        dbg!(value);
    } else {
        println!("No match found");
    }
}
