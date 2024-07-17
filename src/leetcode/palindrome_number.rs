// 回文数
pub(crate) fn test() {
    let mut num = 121;
    println!("{}", is_palindrome(num));
    num = 123;
    println!("{}", is_palindrome(num));

}


pub fn is_palindrome(x: i32) -> bool {
    // parse string
    let s = x.to_string();
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        if s.chars().nth(left) != s.chars().nth(right) {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}