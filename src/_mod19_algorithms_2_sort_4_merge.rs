use chrono::Duration;
// Time Complexity O(n log(n)))
// Space Complexity O(n)
fn sort(items: &mut [i32]) {
    if items.len() == 1 {
        return;
    }
    let middle = items.len() / 2;
    let mut result = items.to_vec();
    sort(&mut items[..middle]);
    sort(&mut items[middle..]);
    merge(&items[..middle], &items[middle..], &mut result);
    items.copy_from_slice(&result);
}
fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let (mut l, mut r, mut i) = (0, 0, 0);
    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            result[i] = left[l];
            l += 1;
        } else {
            result[i] = right[r];
            r += 1;
        }
        i += 1;
    }
    if l < left.len() {
        result[i..].copy_from_slice(&left[l..]);
    }
    if r < right.len() {
        result[i..].copy_from_slice(&right[r..]);
    }
}

pub(crate) fn test() {

    let mut nums = [ 4, 1, 5, 3, 2  ];
    sort(&mut nums);
    // sortItems is {1, 2, 3, 4, 5}
    println!("nums is {:?}", nums);
    // *** simplified speed test ***
    let mut items = [0i32; 200];
    for i in 0..items.len() {
        items[i] = i as i32;
    }
    let tmp = items[5];
    items[5] = items[6];
    items[6] = tmp;
    let count = 1000;
    let start = chrono::offset::Utc::now();
    for _ in 1..count {
        sort(&mut items);
    }
    let now = chrono::offset::Utc::now();
    let delta: Duration = now - start;
    println!("milliseconds is {}",
             delta.num_milliseconds());
    println!("items is {:?}", items);
    // about 127 milliseconds
}