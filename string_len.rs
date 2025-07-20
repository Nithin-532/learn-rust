fn main() {
    let name = String::from("Nithin");
    let len = get_str_len(name);
    println!("the length of the string is {}", len);
}

// Getting length of a string
fn get_str_len(str: String) -> usize {
    str.chars().count() // Implicit return 
}
