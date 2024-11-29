use std::fs;
pub(crate) fn test() {
    main().unwrap();
}
fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/test";
    fs::create_dir(path)?;
    print!("successfully created!");
    Ok(())
}
