use std::{io, fs};
use std::path::Path;
pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "tmp";
    let path_copy = "tmp_copy";
    copy_dir(path, path_copy)?;
    print!("Directory copied successfully!");
    Ok(())
}

fn copy_dir(src: impl AsRef<Path>,
            dst: impl AsRef<Path>) -> io::Result<()>
{
    fs::create_dir_all(&dst)?;
    for obj in fs::read_dir(src)? {
        let obj = obj?;
        let f_type = obj.file_type()?;
        if f_type.is_dir() {
            copy_dir(obj.path(), dst.as_ref()
                .join(obj.file_name()))?;
        } else {
            fs::copy(obj.path(), dst.as_ref()
                .join(obj.file_name()))?;
        }
    }
    Ok(())
}