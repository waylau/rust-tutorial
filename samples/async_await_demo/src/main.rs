/// 实战：`async/await`简单入门
// `block_on`会阻塞当前线程直到指定的`Future`执行完成，
// 这种阻塞当前线程以等待任务完成的方式较为简单、粗暴，
// 好在其它运行时的执行器(executor)会提供更加复杂的行为，
// 例如将多个`future`调度到同一个线程上执行。
use futures::executor::block_on;
use std::thread::sleep;
use std::time::Duration;

async fn hello_world() {
    // 使用`.await`可以等待另一个异步调用的完成
    // 在等待的过程中，该线程还可以继续执行
    hello_cat().await;

    println!("hello, world!");
}

async fn hello_cat() {
    // 模拟耗时操作
    sleep(Duration::from_millis(1000));

    println!("hello, kitty!");
}

fn main() {
    // 返回一个Future, 因此不会打印任何输出
    let future = hello_world(); 

    // 执行`Future`并等待其运行完成，此时"hello, world!"会被打印输出
    block_on(future); 
}
