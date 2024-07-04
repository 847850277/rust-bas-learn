use std::fs;
pub(crate) fn test() {

    let path = "tmp/file.txt";
    fs::remove_file(path).unwrap();
    print!("Deleted!");
    //Ok(())

}