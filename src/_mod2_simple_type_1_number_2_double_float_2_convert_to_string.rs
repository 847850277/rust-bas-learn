pub(crate) fn test() {
    let exp = 2.718281828;

    let s1 = exp.to_string();
    //s1 is 2.718281828

    let s2 = format!("{:.3}", exp);
    //s2 is 2,718

    let s3 = format!("{:.2e}", exp / 100.0);
    //s3 is 2.72e-2

    println!("s1 is \"{s1}\"");
    println!("s2 is \"{s2}\"");
    println!("s3 is \"{s3}\"");

}