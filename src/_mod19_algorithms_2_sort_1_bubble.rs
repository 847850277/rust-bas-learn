use chrono::Duration;
// Time Complexity O(n^2)
// Space Complexity O(1)
fn sort(items: &mut [i32]) {
    for i in 0..items.len() {
        for j in i + 1..items.len() {
            if items[j] < items[i] {
                let tmp = items[j];
                items[j] = items[i];
                items[i] = tmp;
            }
        }
    }
}

pub(crate) fn test() {
    let mut nums = [4, 1, 5, 3, 2];
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
    println!("milliseconds is {}", delta.num_milliseconds());
    println!("items is {:?}", items);
    // about 539 milliseconds
}
