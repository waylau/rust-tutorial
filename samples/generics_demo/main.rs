/// 泛型
fn main() {
    // 未使用泛型
    println!("add i8: {}", add_i8(2i8, 3i8));
    println!("add i32: {}", add_i32(20, 30));
    println!("add f64: {}", add_f64(1.23, 1.23));

    // 在函数定义中使用泛型
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));

    // x、y相同类型
    let i = Point { x: 5, y: 10 };
    dbg!("x: {}, y: {}", i.x, i.y);

    let i = Point { x: 1.0, y: 4.0 };
    dbg!("x: {}, y: {}", i.x, i.y);

    // x、y不同类型
    let i = Point { x: 1, y: 4.0 };
    dbg!("x: {}, y: {}", i.x, i.y);

    // 方法定义中的泛型
    let i = Point { x: 5, y: 10 };
    println!("x: {}, y: {}", i.x(), i.y());

    
    // 数组1，长度2
    let arr1: [i32; 2] = [1, 2];
    display_array(arr1);

    // 数组2，长度3
    let arr2: [i32; 3] = [1, 2, 3];
    display_array(arr2);

    // 数组3，长度3，char型
    let arr3: [char; 3] = ['中', '国', '!'];
    display_array(arr3);

}

fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

/*
fn add<T>(a: T, b: T) -> T {
    a + b
}
 */

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

/*
struct Point<T> {
    x: T,
    y: T,
}
 */

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        .y
    }
}

fn display_array<T, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr.len());
}
