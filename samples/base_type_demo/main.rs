/// 基本类型示例
fn main() {
    // 数值类型
    // 整数
    let a = 12_123;
    println!("a: {a}");

    let a = 0xef;
    println!("a: {a}");

    let a = 0o76;
    println!("a: {a}");

    let a = 0b1011_0100;
    println!("a: {a}");

    let a = b'B';
    println!("a: {a}");

    // 浮点数
    let f = 3.1;
    println!("f: {f}");

    let f: f64 = 3.01;
    println!("f: {f}");

    // 布尔类型
    // 未指定类型，可以推导出布尔类型
    let b = true;
    println!("b: {b}");

    // 指定布尔类型
    let b: bool = false;
    println!("b: {b}");

    // 字符类型
    let c = 'z';
    println!("c: {c}");

    let c = 'ℤ';
    println!("c: {c}");

    let c = '国';
    println!("c: {c}");
		
    let c = '😻';
    println!("c: {c}");
}
