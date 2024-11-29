pub(crate) fn test() {
    let ar = [2, 3, 5];
    let result = get_first_last(&ar);
    //result.first is 2
    //result.last is 5
    println!("first is {:?}", result.0);
    println!("last is {:?}", result.1);
}

fn get_first_last(ar: &[i32]) -> (i32, i32) {
    let mut first = -1;
    let mut last = -1;
    if ar.len() > 0 {
        first = ar[0];
        last = ar[ar.len() - 1];
    }
    return (first, last);
}
