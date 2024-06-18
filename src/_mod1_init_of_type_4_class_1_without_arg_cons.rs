//There are no classes in Rust
#[derive(Debug)]
#[allow(dead_code)]
struct Phone<'a>  {
    model: &'a str
}

#[derive(Debug)]
#[allow(dead_code)]
struct Employee<'a> {
    first_name: &'a str,
    last_name: &'a str,
    phone: Phone<'a>
}

pub fn test(){

    let nokia_phone = Phone{ model: "Nokia 6610" };

    let kim = Employee {
        first_name: "Victorya",
        last_name: "Kim",
        phone: Phone{ model: "iPhone 5" }
    };

    println!("nokia_phone is {:?}", nokia_phone);
    println!("kim is {:?}", kim);
    dbg!(nokia_phone);
    dbg!(kim);
}

