
pub fn number_game(nums: Vec<i32>) -> Vec<i32> {

    let mut copy_array = nums.clone();
    copy_array.sort();
    let mut alice = vec![];
    let mut bob = vec![];
    for i in 0..copy_array.len(){
        if(i % 2 == 0){
            alice.push(copy_array[i]);
        }else {
            bob.push(copy_array[i]);
        }
    }
    let mut result = vec![];
    for i in 0..alice.len() {
        result.push(bob[i]);
        result.push(alice[i]);
    }
    return result;

}

pub(crate) fn test() {
    //let vec1 = vec![5, 4, 2, 3];
    let vec1 = vec![2,5];
    let vec2 = number_game(vec1);
    dbg!(vec2);
}