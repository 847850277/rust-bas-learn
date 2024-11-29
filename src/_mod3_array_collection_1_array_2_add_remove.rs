pub(crate) fn test() {
    //the length of the array type does not change
    //it is recommended to use a class 'vec'
    let mut numbers = vec![2, 5, 7];
    numbers.push(11);
    //numbers is [2, 5, 7, 11]
    print!("{:?}\n", numbers);

    numbers.insert(1, 3);
    //numbers is [2, 3, 5, 7, 11]
    print!("{:?}\n", numbers);

    numbers.remove(2);
    //numbers is [2, 3, 7, 11]
    print!("{:?}\n", numbers);

    numbers.append(&mut vec![13, 17]);
    //primeNumbers is [2, 3, 7, 11, 13, 17]
    print!("{:?}\n", numbers);

    numbers.splice(0..2, [2, 3, 5]);
    //primeNumbers is [2, 3, 5, 7, 11, 13, 17]
    print!("{:?}\n", numbers);

    numbers.clear();
    //primeNumbers is []
    print!("{:?}", numbers);
}
