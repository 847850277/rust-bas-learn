use std::fs::File;
use std::io::{prelude::*, BufReader};
pub(crate) fn test() {
    main().unwrap();
}
fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("Line from file: {}", line?);
    }
    Ok(())
}
