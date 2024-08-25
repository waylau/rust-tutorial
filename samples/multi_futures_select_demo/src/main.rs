/// 使用`select!`
use std::thread::sleep;
use std::time::Duration;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
    executor::block_on
};
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
    let f1 = function1().fuse();
    let f2 = function2().fuse();
 

    pin_mut!(f1, f2);

    select! {
        () = f1 => println!("function1 done"),
        () = f2 => println!("function2 done"),
    }
}

fn main() {
    block_on(async_main());
    println!("end!");
}