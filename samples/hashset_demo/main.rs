/// HashSet
fn main() {
    // 导入包
    use std::collections::HashSet;

    // 创建HashSet，设置类型
    let h_new: HashSet<&str> = HashSet::new();
    println!("len: {}, capacity: {}", h_new.len(), h_new.capacity());

    // 创建HashSet，设置类型、容量
    let h_capacity: HashSet<&str> = HashSet::with_capacity(14);
    println!(
        "len: {}, capacity: {}",
        h_capacity.len(),
        h_capacity.capacity()
    );

    // 创建HashSet
    let h_from: HashSet<&str> = HashSet::from(["Red", "Green", "Blue"]);
    println!("len: {}, capacity: {}", h_from.len(), h_from.capacity());

    // 创建动态数组
    let color_array = vec!["Red", "Green", "Blue"];

    // 创建HashSet，使用collect将动态数组转为HashSet
    let h_collect: HashSet<&str> = color_array.into_iter().collect();

    // 访问HashSet，返回 Option 类型
    let red_value = h_collect.get("Red");
    match red_value {
        Some(red_value) => println!("red_value: {red_value}"),
        None => println!("no element."),
    }

    // 访问HashSet，遍历
    for key in &h_collect {
        println!("{key}");
    }

    // 创建HashSet
    let mut h_new: HashSet<&str> = HashSet::new();

    // 更新HashSet，插入
    h_new.insert("Red");
    h_new.insert("Green");
    h_new.insert("Blue");
    println!("len: {}, capacity: {}", h_new.len(), h_new.capacity());

    // 替换已有的值
    h_new.insert("Blue");
    println!("len: {}, capacity: {}", h_new.len(), h_new.capacity());

    // 访问HashSet，遍历
    for key in &h_new {
        println!("{key}");
    }
}
