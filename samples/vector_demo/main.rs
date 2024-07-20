/// 动态数组
fn main() {
    // 创建动态数组，默认值为 0，初始长度为 3
    let v = vec![0; 3];
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from);

    // 创建动态数组
    let v_vec = vec![0, 0, 0];
    assert_eq!(v_vec, v_from);

    // 创建动态数组，设置类型
    let v_new: Vec<&str> = Vec::new();
    println!("len: {}, capacity: {}", v_new.len(), v_new.capacity());

    // 创建动态数组，设置类型、容量
    let v_with_capacity: Vec<&str> = Vec::with_capacity(10);
    println!(
        "len: {}, capacity: {}",
        v_with_capacity.len(),
        v_with_capacity.capacity()
    );

    // 创建动态数组
    let color_array = vec!["Red", "Green", "Blue"];

    // 访问动态数组
    let a = color_array[1];
    println!("a: {}", a);

    // 访问动态数组，返回 Option 类型
    let opention_a = color_array.get(1);
    match opention_a {
        Some(opention_a) => println!("opention_a: {opention_a}"),
        None => println!("no element."),
    }

    // 访问动态数组，获取动态数组切片
    let s = &color_array[1..3];
    dbg!("s: {}", s);

    // 访问动态数组，遍历
    for c in &color_array {
        println!("c: {c}");
    }

    // 创建动态数组
    let mut month_array: Vec<&str> = Vec::new();

    // 更新动态数组，插入
    month_array.push("January");
    month_array.push("February");
    month_array.push("March");
    month_array.push("April");
    month_array.push("May");
    month_array.push("June");

    // 创建动态数组
    let mut month_array2: Vec<&str> = Vec::new();

    month_array2.push("July");
    month_array2.push("August");
    month_array2.push("September");

    // 更新动态数组，附加动态数组元素
    month_array.append(&mut month_array2);

    for month in &month_array {
        println!("month: {month}");
    }

    // 更新动态数组，扩展数组数据
    let month_array3 = ["October", "November", "December"];
    month_array.extend(month_array3);
    for month in &month_array {
        println!("month: {month}");
    }

    // 更新动态数组，删除指定位置的元素并返回
    let r = month_array.remove(0);
    println!("r: {r}");

    // 更新动态数组，删除并返回尾部的元素
    let p1 = month_array.pop();
    match p1 {
        Some(p1) => println!("p1: {p1}"),
        None => println!("no element."),
    }

    let p2 = month_array.pop();
    match p2 {
        Some(p2) => println!("p2: {p2}"),
        None => println!("no element."),
    }

    // 更新动态数组，截断到指定长度，多余的元素被删除
    let mut numners = Vec::from([1, 2, 3, 4, 5]);
    numners.truncate(2);
    for t in &numners {
        println!("t: {t}");
    }

    // 更新动态数组，保留满足条件的元素，即删除不满足条件的元素
    let mut numners = Vec::from([1, 2, 3, 4, 5]);
    numners.retain(|x| *x > 3);
    for n in &numners {
        println!("n: {n}");
    }

    // 更新动态数组，清空
    let mut numners = Vec::from([1, 2, 3, 4, 5]);
    numners.clear();
    println!("len: {}, capacity: {}", numners.len(), numners.capacity());

    // 排序
    let mut numners = Vec::from([8, 9, 6, 7, 1, 2, 3, 4, 5]);
    numners.sort_unstable();
    for n in &numners {
        println!("n: {n}");
    }
}
