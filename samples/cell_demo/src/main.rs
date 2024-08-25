/// RefCell与内部可变性模式
fn main() {
    // Cell使用示例
    use std::cell::Cell;

    let cell = Cell::new("rust");
    let value = cell.get();
    println!("Initial value: {}", value);

    // 修改数据
    cell.set("waylau.com");

    let new_value = cell.get();
    println!("New value: {}", new_value);

    // RefCell使用示例
    use std::cell::RefCell;

    //  String 没有实现 Copy 特征
    let cell = RefCell::new(String::from("rust"));
    {
        let value = cell.borrow();
        println!("Initial value: {}", *value);
    }
    {
        let mut value = cell.borrow_mut();
        *value = "waylau.com".to_string();
    }
    {
        let new_value = cell.borrow();
        println!("New value: {}", *new_value);
    }
}
