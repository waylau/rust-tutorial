/// 其他模式适用场景
fn main() {
    // while let条件循环
    // 定义一个字符串
    let mut s = String::from("跟老卫学Rust!");

    // pop()删除并返回字符串的最后一个字符
    while let Some(top) = s.pop() {
        println!("{}", top);
    }

    println!("s: {}", s);

    // for 循环
    // 定义一个字符串数组
    let month_array = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // enumerate 方法产生一个迭代器
    // 迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配
    for (index, value) in month_array.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    // let语句
    let x = 5;
    println!("x: {}", x);

    let (x, y, z) = (1, 2, 3);
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 函数参数
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("point: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
