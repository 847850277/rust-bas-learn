struct Man<'a> {
    name: &'a str,
}
impl<'a> Man<'_> {
    fn new(name: &'a str) -> Man<'a> {
        return Man { name: name };
    }
}

pub(crate) fn test() {
    let man = Man::new("Alex");
    //man.name is "Alex"
    println!("name is '{}'", man.name);
}
