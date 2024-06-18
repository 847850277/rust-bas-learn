struct Man<'a> {
    name: &'a str
}
impl<'a> Man<'_> {
    fn new() -> Man<'a> {
        return Man { name: "unknown" };
    }
}

pub(crate) fn test() {

    let man = Man::new();
    println!("name is '{}'", man.name);

}