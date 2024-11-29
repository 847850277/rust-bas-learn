use std::collections::HashMap;

// 两数之和
pub(crate) fn test() {
    let array = vec![2, 7, 11, 15];
    let target = 9;
    println!("{:?}", two_sum(array, target));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::<i32, i32>::new();
    for i in 0..nums.len() {
        let index = i as i32;
        let value = nums[i];
        let diff_value = target - value;
        if let Some(&other_index) = map.get(&diff_value) {
            return vec![other_index, index];
        }
        map.insert(value, index);
    }
    vec![]
}
