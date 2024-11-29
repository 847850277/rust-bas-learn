use std::io::ErrorKind;

fn test_error() -> Result<(), ErrorKind> {
    return Err(ErrorKind::NotFound);
}
fn fn_with_exception() -> Result<(), ErrorKind> {
    test_error()?;
    Ok(())
}

pub(crate) fn test() {
    if let Err(err) = fn_with_exception() {
        println!("{}", err);
    }
}
