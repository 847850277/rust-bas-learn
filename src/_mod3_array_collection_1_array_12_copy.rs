pub(crate) fn test() {
    let mut numbers1 = [ 1, 2, 3, 4, 5 ];
    let numbers2 = numbers1.clone();
    numbers1[0] = 10;
    //numbers2 is [ 1, 2, 3, 4, 5 ]
    print!("numbers2 is {:?}", numbers2);

}