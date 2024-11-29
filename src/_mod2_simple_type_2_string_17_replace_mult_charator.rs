use regex::Regex;
pub(crate) fn test() {
    let str = "1-/[=2=/]-3";
    let separators = "[=/\\[\\]]";
    let re = Regex::new(separators).unwrap();
    let result = re.replace_all(str, "");

    println!("result is \"{result}\"");
}
