use std::fs::File;
use std::io::prelude::*;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let data = [120u8, 64, 97];
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let mut file = File::create(path)?;
    file.write_all(&data)?;
    print!("Successfully written!");
    Ok(())
}