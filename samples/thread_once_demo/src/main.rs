/// 使用调用一次的函数
use std::sync::Once;
use std::thread;

fn main() {
    let mut value: usize = 0;
    let once: Once = Once::new();

    let handle = thread::spawn(move || {
        // i 将在每次迭代中分别取 1, 2, ..., 9
        for i in 1..10 {
            once.call_once(|| {
                value = i;
                println!("number {i} from the spawned thread!");
            });
        }
    });

    // 等待线程的结束
    handle.join().unwrap();

    println!("main thread running completed!");
}
