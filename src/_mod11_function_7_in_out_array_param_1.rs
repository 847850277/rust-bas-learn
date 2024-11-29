pub(crate) fn test() {
    let mut s1 = "A";
    let mut s2 = "B";
    swap(&mut s1, &mut s2);
    //s1 is "B", s2 is "A"
    println!("s1 is {s1}, s2 is {s2}");
}
fn swap<'a>(s1: &mut &'a str, s2: &mut &'a str) {
    (*s1, *s2) = (*s2, *s1);
}
