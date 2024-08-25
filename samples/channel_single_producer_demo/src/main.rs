/// 单发送者单接收者
use std::sync::mpsc;
use std::thread;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个消息, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        let msg = String::from("waylau.com");
        tx.send(msg).unwrap();
    });

    // 在主线程中接收子线程发送的消息并输出
    println!("receive: {}", rx.recv().unwrap());
}
