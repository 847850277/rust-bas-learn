use std::path::Path;
pub(crate) fn test() {

    let path = r"/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/src/_mod16_file_1_basic_1_exist.rs";
    let exists = Path::new(path).exists();
    println!("exists? {}", exists);

}