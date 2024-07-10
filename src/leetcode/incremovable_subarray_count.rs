
// 统计移除递增子数组的数目 I
pub(crate) fn test() {
    let array = vec![1, 2, 3, 4];
    println!("{}", incremovable_subarray_count(array));
}

pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums.len(){
        for j in i..nums.len(){
            if is_increasing(&nums,i,j){
                res+= 1;
            }
        }
    }
    return res;
}

fn is_increasing(nums: &Vec<i32>, l: usize, r: usize) -> bool {
    for i in 1..nums.len(){
        if(i >= l && i <= r + 1){
            continue;
        }
        if(nums[i] <= nums[i - 1]){
            return false;
        }
    }
    if l >= 1 && r + 1 < nums.len() && nums[r + 1] <= nums[l - 1] {
        return false;
    }
    return true;
}