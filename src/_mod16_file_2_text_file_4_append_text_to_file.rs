use std::fs::OpenOptions;
use std::io::prelude::*;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(file, "Line 3")?;
    print!("Text added successfully!");
    Ok(())
}