use itertools::Itertools;
pub(crate) fn test() {

    let numbers1 = [1, 3, 2, 1, 3];
    let unique1: Vec<&i32> = numbers1
        .iter().unique().collect();
    //unique1 is [1, 3, 2]
    let mut numbers2 = [1, 3, 2, 1, 3];
    numbers2.sort();
    let mut unique2 = numbers2.to_vec();
    unique2.dedup_by_key(|i| *i);
    //unique2 is [1, 2, 3]
    print!("unique1 is {:?}\n", unique1);
    print!("unique2 is {:?}", unique2);

}