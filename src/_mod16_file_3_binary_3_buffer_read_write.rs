use std::io;
use std::io::prelude::*;
use std::fs::File;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let path_copy = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file_copy.out";
    let mut f = File::open(path)?;
    let mut f_copy = File::create(path_copy)?;
    let mut buffer = [0; 1024];
    while {
        let n = f.read(&mut buffer[..])?;
        f_copy.write_all(&buffer[..n])?;
        n > 0
    } {}
    Ok(())
}