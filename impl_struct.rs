struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 { // self -> current struct(1st argument)
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width * self.height)
    }

    fn debug() -> i32 { // just like static function in classes
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20
    };

    println!("area is {}", rect1.area());
    println!("area is {}", rect1.perimeter());
    println!("area is {}", Rect::debug());
}
