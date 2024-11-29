use std::collections::HashMap;
use std::fs::File;

pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let dic = HashMap::from([(1, "one"), (2, "two")]);
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let file = File::create(path)?;
    serde_json::to_writer(file, &dic)?;
    print!("Successfully written!");
    Ok(())
}
