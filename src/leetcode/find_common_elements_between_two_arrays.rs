// 找到两个数组中的公共元素
pub(crate) fn test() {
    todo!()
}

pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut res1 = 0;
    let mut res2 = 0;
    for x1 in nums1.iter() {
        for x2 in nums2.iter() {
            if x1 == x2 {
                res1 += 1;
                break;
            }
        }
    }
    for x2 in nums2.iter() {
        for x1 in nums1.iter() {
            if x2 == x1 {
                res2 += 1;
                break;
            }
        }
    }
    [res1, res2].to_vec()
}
