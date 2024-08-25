/// 使用`join!`
use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;

async fn function1() {
    // 模拟耗时操作
    sleep(Duration::from_millis(2000));
    println!("function1");
}

async fn function2() {
    // 模拟耗时操作
    sleep(Duration::from_millis(1000));
    println!("function2");
}

async fn async_main() {
    let f1 = function1();
    let f2 = function2();

    // 使用join则会并发执行f1和f2
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
    println!("end!");
}