#[derive(Debug)]
enum VecError {
    NoneError,
    EmptyError,
}
fn throw_when_null_or_empty(vector: Option<Vec<i32>>) -> Result<(), VecError> {
    if vector == None {
        return Err(VecError::NoneError);
    }
    if vector.unwrap().len() == 0 {
        return Err(VecError::EmptyError);
    }
    return Ok(());
}

pub(crate) fn test() {

    if let Err(err) = throw_when_null_or_empty(None) {
        match err {
            VecError::NoneError  =>
                println!("Vector is not specified!"),
            VecError::EmptyError =>
                println!("Vector is empty!"),
        }
    }
    if let Ok(_) =  throw_when_null_or_empty(Some(vec![5])) {
        println!("No error has occurred!");
    }

}