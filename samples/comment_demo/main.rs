/// 注释示例
// main函数是程序入口
fn main() {
    /*
    // 执行函数
    println_hello();
    */

    // 获取函数的返回值
    let a: u64 = 1; // 第一个参数
    let b: u64 = 1; // 第二个参数
    let add_result = add(a, b);
    println!("add result: {add_result}");
}

/**
 * 定义带返回的函数
 * a：第一个参数
 * b: 第二个参数
 */
fn add(a: u64, b: u64) -> u64 {
    a + b
}
