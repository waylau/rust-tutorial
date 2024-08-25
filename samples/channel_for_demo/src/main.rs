/// 接收多条消息
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // i 将在每次迭代中分别取 1, 2, ..., 9
        for i in 1..10 {
            let msg = format!("msg {}", i);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 在主线程中接收子线程发送的消息并输出
    for received in rx {
        println!("receive: {received}");
    }
}
