    use std::fmt::Debug;
    use ::http_body::Body;
    use http_body_util::{BodyExt, Full};

use bytes::Bytes;
pub mod http_body;

pub(crate) fn main() {
    println!("mod.rs");

    let full = Full::<Bytes>::from("Hello, World!");
    let body = full.boxed();
    println!("{:?}", body);

    //fmt
    let body1 = body.boxed_unsync();
    println!("{:?}", body1);


}