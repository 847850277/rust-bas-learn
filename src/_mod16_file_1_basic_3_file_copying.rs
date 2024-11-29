use std::fs;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let file_path = r"tmp/file.txt";
    let file_path_to = r"tmp/file_copy.txt";
    fs::copy(file_path, file_path_to)?;
    Ok(())
}
