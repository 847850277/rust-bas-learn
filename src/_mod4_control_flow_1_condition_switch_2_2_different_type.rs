use std::option;

pub(crate) fn test() {

    for i in 1..10 {
        //let e = "3*3";
        let option = get_operaton(i);
        let e = format!("3{}3", option);
        match e.as_str() {
            "3 + 3" => println!("result = 6"),
            "3 - 3" => println!("result = 0"),
            "3 * 3"   => println!("result = 9"),
            "3 / 3"   => println!("result = 1"),
            _   => print!("unknown")
        }
    }

}

fn get_operaton(i: i32) -> &'static str {
        // if(i % 1 == 0){
        //     return " + ";
        // }
        if(i % 2 == 0){
        return " - ";
        }
        if(i % 3 == 0){
        return " * ";
        }
        if(i % 4 == 0){
        return " / ";
        }
        return " + ";

}