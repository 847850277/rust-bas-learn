//There are no classes in Rust
#[derive(Debug)]
struct Resources;
impl Drop for Resources {
    // It called automatically
    // when struct instance is deallocated
    fn drop(&mut self) {
        // release allocated resources
        // eg close the connection to the database,
        // close files, etc.
        println!("Deallocated!");
    }
}

pub(crate) fn test() {

    let _res = Resources;
    println!("{:?}", _res);

}