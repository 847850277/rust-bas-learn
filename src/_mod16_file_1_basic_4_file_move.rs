use std::fs;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let file_path = "tmp/file.txt";
    let file_path_to = "tmp/file_new.txt";
    fs::rename(file_path, file_path_to)?;
    print!("File moved!");
    Ok(())
}