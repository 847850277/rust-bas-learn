use std::fs;
pub(crate) fn test() {
    let dir = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/";
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        println!("{:?}", path.unwrap().file_name())
    }
}
