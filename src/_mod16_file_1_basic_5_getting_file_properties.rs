use std::fs;
use std::path::Path;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let file_path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/src/_mod16_file_1_basic_5_getting_file_properties.rs";
    let metadata = fs::metadata(file_path)?;
    let path = Path::new(file_path);
    //file size
    let file_size = metadata.len();
    //file modification date
    let date_changes = metadata.created()?;
    //file creation date
    let creation_date = metadata.modified()?;
    //is readonly
    let is_readonly = metadata.permissions().readonly();
    //file extension
    let extension = path.extension().unwrap();
    //file name
    let file_name = path.file_name().unwrap();
    //file name without extension
    let file_name_only = path.file_stem().unwrap();
    //file directory
    let file_dir = path.parent().unwrap();
    println!("file_size is {}", file_size);
    let datetime: DateTime<Utc> = date_changes.into();
    println!("date_changes is {}", datetime.format("%Y-%m-%d"));
    println!("creation_date is {:?}", creation_date);
    println!("is_readonly is {}", is_readonly);
    println!("extension is {:?}", extension);
    println!("file_name is {:?}", file_name);
    println!("file_name_only is {:?}", file_name_only);
    println!("file_dir is {:?}", file_dir);
    Ok(())
}
