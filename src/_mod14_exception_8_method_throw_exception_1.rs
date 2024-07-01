pub(crate) fn test() {
    //any method can throw an panic error
    fn method_with_panic() {
        panic!("test panic");
    }
    use std::io::ErrorKind;
    //any method can return error
    fn method_with_error() -> Result<(), ErrorKind> {
        return Err(ErrorKind::NotFound);
    }
}