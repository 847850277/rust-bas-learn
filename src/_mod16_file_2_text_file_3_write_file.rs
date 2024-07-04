use std::fs;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt";
    fs::write(path, "Line 1\nLine 2")?;
    print!("Successfully written!");
    Ok(())
}