/// 将move闭包与线程一同使用
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 增加 move 关键字，强制闭包获取其使用的值的所有权
    let handle = thread::spawn(move || {
        println!("v: {v:?}");
    });

    handle.join().unwrap();
}
