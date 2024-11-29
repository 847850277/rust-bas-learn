use std::fs;
pub(crate) fn test() {
    main().unwrap();
}
fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt";
    let text = fs::read_to_string(path)?;
    print!("text:\n{text}");
    Ok(())
}
