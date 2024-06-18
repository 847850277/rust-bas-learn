use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Employee<'a> {
    first_name: &'a str,
    last_name: &'a str
}

pub fn test(){
    //Map<String, String>
    let languages = HashMap::from([
        ("ru", "russian"),
        ("en", "english")
    ]);

    //Map<Int, String>
    let numbers = HashMap::from([
        (1, "one"), (2, "two")
    ]);

    //Array of Employee
    let employees = HashMap::from([
        (1, Employee{first_name: "Anton",
            last_name: "Pavlov"}),
        (2, Employee{first_name: "Elena",
            last_name: "Kirienko"})
    ]);

    println!("languages is {:?}", languages);
    println!("numbers is {:?}", numbers);
    println!("employees is {:?}", employees);

    dbg!(languages);
    dbg!(numbers);
    dbg!(employees);
}

