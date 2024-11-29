pub(crate) fn test() {
    let ar = [2, 3, 5, 7];
    for (i, v) in ar.iter().enumerate() {
        println!("numbers[{i}] = {v}");
    }
    println!();
    let lines = "one\ntwo\nthree".lines();
    for (i, line) in lines.enumerate() {
        println!("line {}: '{}'", i + 1, line);
    }
}
