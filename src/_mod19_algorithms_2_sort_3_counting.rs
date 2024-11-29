use chrono::Duration;
// Time Complexity O(n^2)
// Space Complexity O(1)
fn sort(items: &mut [i32]) {
    let min = *items.iter().min().unwrap();
    let max = *items.iter().max().unwrap();
    let count: usize = (max - min + 1) as usize;
    let mut counts = vec![0; count];
    for x in items.iter() {
        let n = (*x - min) as usize;
        counts[n] += 1;
    }
    let mut total = 0;
    for i in min..max + 1 {
        let n = (i - min) as usize;
        let old_count = counts[n];
        counts[n] = total;
        total += old_count;
    }
    let mut sorted = vec![0; items.len()];
    for x in items.iter() {
        let n = (*x - min) as usize;
        sorted[counts[n]] = *x;
        counts[n] += 1;
    }
    items.copy_from_slice(&sorted);
}

pub(crate) fn test() {
    let mut nums = [4, 1, 5, 3, 2];
    let sorted_nums = sort(&mut nums);
    // nums is {1, 2, 3, 4, 5}
    println!("sorted_nums is {:?}", sorted_nums);
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
    println!("sorted_items is {:?}", sort(&mut items));
    // about 130 milliseconds
}
