/// 载歌载舞
use futures::executor::block_on;

async fn sing_song() {
    println!("I am singing");
}

async fn dance() {
    println!("I am dancing");
}

async fn async_main() {
    let f1 = sing_song();
    let f2 = dance();

    // `join!`可以并发的处理和等待多个`Future`
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
