pub(crate) fn test() {
    let str = "123";
    let pad_start1 = format!("{: >10}", str);
    // pad_start1 is '       123'

    let pad_start2 = format!("{:0>10}", str);
    // padStart2 is '0000000123'

    let pad_end1 = format!("{: <10}", str);
    // padEnd1 is '123       '

    let pad_end2 = format!("{:*<10}", str);
    // padEnd2 is '123*******'

    println!("pad_start1 is '{pad_start1}'");
    println!("pad_start2 is '{pad_start2}'");
    println!("pad_end1 is '{pad_end1}'");
    println!("pad_end2 is '{pad_end2}'");
}
