/// 面向对象的图形
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area<T: Shape>(shape: &T) {
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };

    print_area(&circle);
    print_area(&rectangle);
}
