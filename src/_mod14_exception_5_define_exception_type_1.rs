use std::{error::Error, fmt};
#[derive(Debug)]
struct SimpleError;
impl Error for SimpleError {}
impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Something happened!");
    }
}
fn show_error() -> Result<(), SimpleError> {
    return Err(SimpleError);
}

pub(crate) fn test() {
    if let Err(err) = show_error() {
        println!("{}", err.to_string());
    }
}
