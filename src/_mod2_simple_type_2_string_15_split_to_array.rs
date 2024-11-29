pub(crate) fn test() {
    let str_data = "1981|Kim Victorya|engineer";
    let split_data = str_data.split("|");
    let arr_data = split_data.collect::<Vec<&str>>();
    let year = arr_data[0].parse::<i16>().unwrap();
    //year is 1981
    let name = arr_data[1];
    //name is "Kim Victorya"
    let position = arr_data[2];
    //position is "engineer"

    println!("year is {year}");
    println!("name is \"{name}\"");
    println!("position is \"{position}\"");
}
