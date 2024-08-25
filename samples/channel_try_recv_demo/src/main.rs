/// 非阻塞接收消息
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个消息通道, 返回一个元组：(发送者，接收者)
    let (tx, rx) = mpsc::channel();

    // 创建线程，并发送消息
    thread::spawn(move || {
        // 发送一个消息, send方法返回Result<T,E>，通过unwrap进行快速错误处理
        let msg = String::from("waylau.com");
        tx.send(msg).unwrap();
    });

    // 在主线程中非阻塞的接收子线程发送的消息并输出
    println!("receive: {:?}", rx.try_recv());

    // 延迟1秒
    thread::sleep(Duration::from_millis(1000));
    // 在主线程中非阻塞的接收子线程发送的消息并输出
    println!("1s late receive: {:?}", rx.try_recv());

    // 延迟1秒
    thread::sleep(Duration::from_millis(1000));
    // 在主线程中非阻塞的接收子线程发送的消息并输出
    println!("2s late receive: {:?}", rx.try_recv())
}
