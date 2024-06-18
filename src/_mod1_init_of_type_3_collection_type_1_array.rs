#[derive(Debug)]
#[allow(dead_code)]
struct Employee<'a> {
    first_name: &'a str,
    last_name: &'a str
}

pub fn test(){
    //Array of integer
    let prime_numbers = [ 2, 3, 5, 7, 11, 13, 17, 19 ];

    //Array of string
    let game_list = [ "soccer", "hockey", "basketball" ];

    //Array of Employee
    let employees = [
        Employee{first_name: "Anton", last_name: "Pavlov"},
        Employee{first_name: "Elena", last_name: "Kirienko"}
    ];

    println!("prime_numbers is {:?}", prime_numbers);
    println!("game_list is {:?}", game_list);
    println!("employees is {:?}", employees);

    dbg!(prime_numbers);
    dbg!(game_list);
    dbg!(employees);
}

