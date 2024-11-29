pub(crate) fn test() {
    let s1 = "three";
    let s2 = "two";
    let s3: String = "one".to_owned();
    let mut s4 = s1.to_owned() + ", ";
    s4.push_str(s2);
    s4.push_str(", ");
    s4.push_str(&s3);
    let mut s_go = format!("{s4}, go");
    s_go = [&s_go, "!"].concat();
    // sGo is "three, two, one, go!"

    print!("{s_go}");
}
