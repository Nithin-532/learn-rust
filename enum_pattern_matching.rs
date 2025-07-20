enum Shape {
    Rectangle(f64, f64), // width, height
    Circle(f64), // radius
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    let rect_area = print_area(rect);
    let circle = Shape::Circle(1.0);
    let circle_area = print_area(circle);
    println!("{}, {}",rect_area, circle_area)
}

fn print_area(shape: Shape) -> f64 {
    // pattern matching
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
