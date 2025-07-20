fn main() {
    println!("{}", is_even(21));
}

// Finding whether a number is even or not
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
