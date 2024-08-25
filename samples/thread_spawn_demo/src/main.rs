/// thread::spawn 创建线程
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        // i 将在每次迭代中分别取 1, 2, ..., 9
        for i in 1..10 {
            println!("number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}