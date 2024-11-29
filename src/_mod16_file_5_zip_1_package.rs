use std::fs::File;
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};

pub(crate) fn test() {
    //[dependencies]
    //zip = "0.6.4"
    main().unwrap();
}

fn main() -> ZipResult<()> {
    let source_path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/file.txt";
    let zip_file = source_path.to_owned() + ".zip";
    let file = File::create(zip_file)?;
    let mut writer = ZipWriter::new(file);
    //writer.start_file(source_path, FileOptions::default())?;
    writer.start_file::<_, (), _>(source_path, FileOptions::default())?;
    writer.finish()?;
    Ok(())
}
