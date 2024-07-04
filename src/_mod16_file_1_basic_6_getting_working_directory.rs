use std::env;
pub(crate) fn test() {

    let path = env::current_dir();
    match path {
        Ok(p) => println!("path: {p:?}"),
        Err(e) => println!("error: {e:?}"),
    }

}