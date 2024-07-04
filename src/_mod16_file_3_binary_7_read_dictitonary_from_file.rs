use std::collections::HashMap;
use std::fs;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.out";
    let text = fs::read_to_string(path)?;
    let dic: HashMap::<u8, &str> =
        serde_json::from_str(&text)?;
    print!("dic is {:?}", dic);
    Ok(())
}