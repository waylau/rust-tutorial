/// 单线程中使用互斥器
use std::sync::Mutex;

fn main() {
    // 使用 Mutex 结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);

    {
        // 获取锁，然后解引用为 m 的引用
        // lock() 返回的是 Result
        let mut num = m.lock().unwrap();
        *num = 6;

        // 锁自动被drop
    }

    println!("m = {:?}", m);
}
