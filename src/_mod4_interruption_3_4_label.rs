pub(crate) fn test() {
    let mut first_match_value = -1;
    let array1 = [1, 2, 3];
    let array2 = [2, 3, 4];
    'first_loop: for i in array1 {
        for n in array2 {
            if i == n {
                first_match_value = n;
                break 'first_loop;
            }
        }
    }
    //first_match_value is 2
    print!("{}", first_match_value);

}