use std::fs;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/test";
    fs::remove_dir(path)?;
    print!("successfully deleted!");
    Ok(())
}