/// Rc使用实例
fn main() {
    // Rc使用示例
    use std::rc::Rc;

    let a = Rc::new(String::from("waylau.com"));
    assert_eq!(1, Rc::strong_count(&a));

    let b = Rc::clone(&a);

    // Rc::strong_count 获取引用计数的关联函数
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));

    // Ac使用示例
    use std::sync::Arc;
    use std::thread;

    let s = Arc::new(String::from("waylau.com"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s)
        });
    }
}


