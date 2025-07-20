fn main() {
    println!("{}", fib(3));
}

fn fib(n: u32) -> u32 {
    let mut first: u32 = 0;
    let mut second: u32 = 1;
    if n == 0 {
        return first;
    }
    if n == 1 {
        return second;
    }
    for _ in 1..n {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
