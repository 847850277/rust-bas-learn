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

impl Phone<'_> {
    fn new(model: &str) -> Phone {
        return Phone { model: model };
    }
}

impl Employee<'_> {
    fn new<'a>(first_name: &'a str, last_name:
    &'a str, phone: Phone<'a>) -> Employee<'a> {
        return Employee {
            first_name: first_name,
            last_name: last_name,
            phone: phone
        };
    }
}

pub fn test(){
    let nokia_phone = Phone::new("Nokia 6610");

    let kim = Employee::new("Victorya", "Kim",
                            Phone::new("iPhone 5"));

    println!("nokia_phone is {:?}", nokia_phone);
    println!("kim is {:?}", kim);
    dbg!(nokia_phone);
    dbg!(kim);
}


