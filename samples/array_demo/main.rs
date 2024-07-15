/// 枚举
fn main() {
    // 创建一个三原色数组
    let color_array: [&str; 3] = ["Red", "Green", "Blue"];

    // 访问数组
    let a = color_array[1];
    println!("a: {}", a);

    // 省略了数组的类型及长度
    let color_array = ["Red", "Green", "Blue"];

    // 访问数组
    let a = color_array[1];
    println!("a: {}", a);

    // 创建一个每个元素都为相同值的数组
    // 等同于 let a = [3, 3, 3, 3, 3];
    let n = [3; 5];

    // 访问数组
    let b = n[0];
    println!("b: {}", b);

    // 数组切片
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

    // 获取数组切片
    let s = &month_array[1..3];
    dbg!("s: {}", s);
}