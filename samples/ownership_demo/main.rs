/// 所有权概述
fn main() {
    // 变量作用域
    {                           // s 在这里无效，它尚未声明
        let s = "waylau.com";   // 从此处起，s 是有效的
    
        // 使用 s
		println!("s: {s}"); 
    }                           // 此作用域已结束，s不再有效

    // 可变的String类型
    let mut s = String::from("waylau.com");
    s.push_str(", very good!"); // push_str() 在字符串后追加字面值
    println!("s: {s}"); // 将打印 s: waylau.com very good!

    // 移动
    let x = 5;
    let y = x;
    println!("x: {x}, y: {y}");

	/*
    let s1 = String::from("hello");
    let s2 = s1; 
    println!("s1: {s1}"); // 错误！s1已经失效
	println!("s2: {s2}");
	*/
	
    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 克隆
    println!("s1: {s1}"); // s1仍然有效
    println!("s2: {s2}"); 
}
