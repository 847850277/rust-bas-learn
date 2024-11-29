use std::fs;
use std::fs::File;
use std::io::prelude::*;
use zip::result::ZipResult;
use zip::ZipArchive;
fn main() -> ZipResult<()> {
    let zip_file = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt.zip";
    let file = File::open(zip_file)?;
    let mut zip = ZipArchive::new(file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let file_path = "./".to_owned() + file.name();
        if file.is_dir() {
            fs::create_dir_all(file_path)?;
        } else {
            let mut writer: Vec<u8> = vec![];
            std::io::copy(&mut file, &mut writer)?;
            let mut file_to = File::create(file_path)?;
            file_to.write_all(&writer)?;
        }
    }
    Ok(())
}

pub(crate) fn test() {
    main().unwrap();
}
