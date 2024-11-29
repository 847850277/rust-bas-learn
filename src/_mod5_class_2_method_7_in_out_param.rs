struct Swap {}
impl Swap {
    fn strings<'a>(s1: &mut &'a str, s2: &mut &'a str) {
        let v1: &str = &*s1;
        let v2: &str = &*s2;
        (*s1, *s2) = (v2, v1);
    }
}
pub(crate) fn test() {
    let mut s1 = "A";
    let mut s2 = "B";
    Swap::strings(&mut s1, &mut s2);
    //s1 is "B", s2 is "A"
    println!("s1 is {s1}, s2 is {s2}");
}
