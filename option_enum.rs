/*
    enum Option<T> {
        Some(T),
        None
    }
*/

fn main() {
    let index = find_first_a(String::from("jayasree"));

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}
