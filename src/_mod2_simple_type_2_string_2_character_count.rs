pub(crate) fn test() {
    let string_reverse = reverse("string");
    // string_reverse is "gnirts"

    println!("string_reverse is \"{}\"", string_reverse);
}

fn reverse(word: &str) -> String {
    // Characters count
    let char_count = word.chars().count();

    let mut i = char_count;
    let mut result = String::new();
    let chars: Vec<char> = word.chars().collect();
    while i > 0 {
        i = i - 1;
        result += &chars[i].to_string();
    }

    return result;
}
