/// panic!
fn main() {
    /*
    // 调用 panic! 宏
    panic!("Create some panic.");
    */

    // 定义动态数组
    let color_array = vec!["Red", "Green", "Blue"];

    // 数组越界
    let color = color_array[4];
    println!("color: {}", color);
}
