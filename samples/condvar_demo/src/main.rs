/// 使用条件变量
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个包含Mutex和Condvar的元组，并用Arc包装以便在多个线程间共享
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // 生产者线程
    thread::spawn(move || {
        let (lock, cvar) = &*pair_clone;
        let mut started = lock.lock().unwrap();

        println!("producer working before");
        // 模拟工作，比如等待一段时间
        thread::sleep(Duration::from_secs(3));

        // 修改条件并通知消费者
        *started = true;
        cvar.notify_one(); // 只唤醒一个等待的线程
        println!("producer working after");
    });

    // 消费者线程
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();

    // 等待条件成立
    while !*started {
        started = cvar.wait(started).unwrap(); // 释放锁并等待，直到被唤醒
        println!("consumer working");
    }

    println!("done");
}
