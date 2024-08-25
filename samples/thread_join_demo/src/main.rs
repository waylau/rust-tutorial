/// 等待子线程结束
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        // i 将在每次迭代中分别取 1, 2, ..., 9
        for i in 1..10 {
            println!("number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 等待线程的结束
    handle.join().unwrap();

    for i in 1..5 {
        println!("number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
