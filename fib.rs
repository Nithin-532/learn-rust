fn main() {
    let x: i32 = 46;
    println!("{}", fib(x));
}

fn fib(n: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;
    if n == 0 {
        return first;
    }
    if n == 1 {
        return second;
    }
    for i in 1..n - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}
