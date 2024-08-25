use kv_store::KeyValueStore;

fn main() {
    // 初始化数据库
    let db = KeyValueStore::new();

    // 设置数据
    db.set("key1".to_string(), "waylau.com".to_string());

    // 获取存在的数据
    let optionValue1 = db.get("key1");

    match optionValue1 {
        Some(value) => println!("key1 value: {}", value),
        None => println!("key1 value: None"),
    }

    // 获取不存在的数据
    let optionValue2 = db.get("key2");

    match optionValue2 {
        Some(value) => println!("key2 value: {}", value),
        None => println!("key2 value: None"),
    }
}
