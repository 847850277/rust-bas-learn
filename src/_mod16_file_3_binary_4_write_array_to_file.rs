use std::fs::File;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let numbers = [1, 2, -3];
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let file = File::create(path)?;
    serde_json::to_writer(file, &numbers)?;
    print!("Successfully written!");
    Ok(())
}