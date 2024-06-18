use std::collections::HashMap;

pub(crate) fn test() {
    let mut numbers = [1i16, 2, 3, 4, 5];
    numbers.sort_by(|a, b| (a % 2).cmp(&(b % 2)));
    let mut dic: HashMap<&str, Vec<i16>> = HashMap::new();
    for &number in numbers.iter() {
        let key = if number % 2 == 0 { "even" } else { "odd" };
        dic.entry(key).or_insert_with(Vec::new).push(number);
    }
    println!("dic is {:?}", dic);
}