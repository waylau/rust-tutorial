/// 将任何类型转换成String
fn main() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        // 实现 fmt 方法
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // 自定义格式，使得仅显示 `x` 和 `y` 的值。
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let point = Point { x: 1, y: 2 };
    println!("point: {}", point);
}
