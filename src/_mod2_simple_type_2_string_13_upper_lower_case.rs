pub(crate) fn test() {
    let str = "Lower And Upper";

    let lower = str.to_lowercase();
    //lower is "lower and upper"

    let upper = str.to_uppercase();
    //upper is "LOWER AND UPPER"

    let capitalize = capitalize(&lower);
    //capitalize is "Lower and upper"

    println!("lower is \"{lower}\"");
    println!("upper is \"{upper}\"");
    println!("capitalize is \"{capitalize}\"");

    pub fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        println!("{:?}", c);
        //let option = c.next();
        //println!("{:?}", option);
        // c.next 表示当前第一个字符串
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase()
                .collect::<String>() + c.as_str(),
        }
    }

}