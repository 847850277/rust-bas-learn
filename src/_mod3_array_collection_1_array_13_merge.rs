pub(crate) fn test() {
    let numbers1 = [2, 3, 5];
    let numbers2 = [7, 11, 13];
    let all_numbers = [numbers1, numbers2]
        .concat();
    //all_numbers is [2, 3, 5, 7, 11, 13]
    println!("all_numbers is {:?}", all_numbers);

}