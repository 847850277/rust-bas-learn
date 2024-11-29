enum ProgramInfo {
    Version(i32, i32, i32),
    Name(String),
}
fn print_info(info: ProgramInfo) {
    if let ProgramInfo::Version(major, minor, build) = info {
        println!("version: {major}.{minor}.{build}");
    }
    if let ProgramInfo::Name(name) = info {
        println!("name: {name}");
    }
}

pub(crate) fn test() {
    let info1 = ProgramInfo::Name("CCDEditor".to_string());
    print_info(info1);
    let info2 = ProgramInfo::Version(3, 5, 25467);
    print_info(info2);
}
