#[derive(Debug)]
enum VecError {
    NoneError,
    EmptyError,
}

pub(crate) fn test() {
    if let Err(err) = throw_when_null_or_empty(None) {
        println!("Error happened: {:?}", err);
    }
    let vec1 = Some(vec![]);
    if let Err(err) = throw_when_null_or_empty(vec1) {
        println!("Error happened: {:?}", err);
    }
}

fn throw_when_null_or_empty(vector: Option<Vec<i32>>) ->
Result<(), VecError> {
    if vector == None {
        return Err(VecError::NoneError);
    }
    if vector.unwrap().len() == 0 {
        return Err(VecError::EmptyError);
    }
    return Ok(());
}