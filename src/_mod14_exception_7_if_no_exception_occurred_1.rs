use std::io::ErrorKind;
fn error_if_true(param: bool) -> Result<(), ErrorKind> {
    if param {
        return Err(ErrorKind::NotFound);
    } else {
        return Ok(());
    }
}
pub(crate) fn test() {
    let result = error_if_true(false);
    let _ = match result {
        Ok(_) => print!("No error!"),
        Err(error) => panic!("Error: {:?}", error),
    };
}
