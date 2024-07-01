use regex::Regex;
pub(crate) fn test() {

    let str_data = "1981|Kim Victorya|engineer";
    let pattern = r"(?<year>\d{4})\|(?<n>[\w\s]+)\|(?<p>.+)";
    let re = Regex::new(pattern).unwrap();
    if let Some(caps) = re.captures(str_data) {
        let year = &caps["year"];
        //year is 1981
        let full_name = &caps["n"];
        //name is "Kim Victorya"
        let position = &caps["p"];
        //position is "engineer"
        println!("year is {year}");
        println!("name is '{full_name}'");
        println!("position is '{position}'");
    };

}