/// HashMap
fn main() {
    // 导入包
    use std::collections::HashMap;

    // 创建HashMap，设置类型
    let h_new: HashMap<&str, i32> = HashMap::new();
    println!("len: {}, capacity: {}", h_new.len(), h_new.capacity());

    // 创建HashMap，设置类型、容量
    let h_capacity: HashMap<&str, i32> = HashMap::with_capacity(14);
    println!("len: {}, capacity: {}", h_capacity.len(), h_capacity.capacity());

    // 创建HashMap
    let h_from: HashMap<&str, i32> = HashMap::from([("Red", 11), ("Green", 23), ("Blue", 45)]);
    println!("len: {}, capacity: {}", h_from.len(), h_from.capacity());

    // 创建动态数组
    let color_array = vec![("Red", 11), ("Green", 23), ("Blue", 45)];

    // 创建HashMap，使用collect将动态数组转为HashMap
    let h_collect: HashMap<&str, i32> = color_array.into_iter().collect();

    // 访问HashMap，返回 Option 类型
    let red_value = h_collect.get("Red");
    match red_value {
        Some(red_value) => println!("red_value: {red_value}"),
        None => println!("no element."),
    }

    // 访问HashMap，遍历
    for (key, value) in &h_collect {
        println!("{key}: {value}");
    }

    // 创建HashMap
    let mut h_new: HashMap<&str, i32> = HashMap::new();

    // 更新HashMap，插入
    h_new.insert("Red", 11);
    h_new.insert("Green", 23);
    h_new.insert("Blue", 45);

    // 替换已有的值
    let old = h_new.insert("Blue", 20);
    match old {
        Some(old) => println!("old: {old}"),
        None => println!("no element."),
    }

    // 访问HashMap，遍历
    for (key, value) in &h_new {
        println!("{key}: {value}");
    }
}
