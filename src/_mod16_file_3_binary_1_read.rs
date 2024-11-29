use std::fs::File;
use std::io::prelude::*;

pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let mut file = File::open(path)?;
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data)?;
    println!("data is {:?}", data);
    Ok(())
}
