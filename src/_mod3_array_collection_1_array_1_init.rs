pub(crate) fn test() {
    //Empty vector
    let n1 = Vec::<u32>::new();
    let n2: Vec<i32> = vec![];

    // Single-dimensional array
    let n3 = [1i32, 2, 3];
    let s1 = ["1", "2", "3"];

    // Multidimensional array.
    let mut n4 = [[1, 2, 3], [4, 5, 6]];
    n4[1][2] = 7;

    // Jagged array
    let mut n5 = [vec![1, 2], vec![3, 4, 5]];
    n5[1][2] = 7;

    println!("n1 is {:?}", n1);
    println!("n2 is {:?}", n2);
    println!("n3 is {:?}", n3);
    println!("s1 is {:?}", s1);
    println!("n4 is {:?}", n4);
    println!("n5 is {:?}", n5);
}
