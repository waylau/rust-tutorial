/// 使用lazy_static
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref LANG_DB: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "Java");
        m.insert(1, "Rust");
        m.insert(2, "C");
        m
    };
}

fn main() {
    // 首次访问`LANG_DB`的同时对其进行初始化
    println!("The entry for `0` is \"{}\".", LANG_DB.get(&0).unwrap());

    // 后续的访问仅仅获取值，再不会进行任何初始化操作
    println!("The entry for `1` is \"{}\".", LANG_DB.get(&1).unwrap());
}
