pub(crate) fn test() {
    let ar = [1, 2, 3];
    let is_contain2 = contain_number(&ar, &2);
    //is_contain2 is true
    let is_contain4 = contain_number(&ar, &4);
    //is_contain4 is false
    println!("is_contain2 is {is_contain2}");
    println!("is_contain4 is {is_contain4}");
}

fn contain_number(ar: &[i32], i: &i32) -> bool {
    for n in ar {
        if n == i {
            return true;
        }
    }
    return false;
}
