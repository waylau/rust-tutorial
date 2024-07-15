/// 函数示例
// main函数是程序入口
fn main() {
    // 执行函数
    println_hello();

    // 执行函数传递参数
    let text = 999;
    println_text(text);

    // 获取函数的返回值
    let a: u64 = 1;
    let b: u64 = 1;
    let add_result = add(a, b);
    println!("add result: {add_result}");
}

// 自定义函数
fn println_hello() {
    // 打印Hello World!
    println!("Hello World!");
}

// 如果函数定义了参数，则参数都需要标注类型
fn println_text(text: u64) {
    println!("text: {text}");
}

// 定义带返回的函数
fn add(a: u64, b: u64) -> u64 {
    // 等同于 
    // return a + b;
    a + b
}
