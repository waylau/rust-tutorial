/// 同步通道
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    // 指定同步通道的消息缓存条数为0
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("send msg before");

        // 发送一个消息
        let msg = String::from("waylau.com");
        tx.send(msg).unwrap();

        println!("send msg after");
    });

    // 延迟3秒
    thread::sleep(Duration::from_secs(3));

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}
