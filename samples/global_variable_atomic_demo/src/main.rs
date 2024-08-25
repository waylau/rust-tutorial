/// 使用原子类型
use std::sync::atomic::{AtomicUsize, Ordering};

// 一天的毫秒数
static ONE_DAY_MS: usize = 24 * 60 * 60 * 1000;

// 毫秒数总计
static TOTAL_MS: AtomicUsize = AtomicUsize::new(0);

fn main() {
    // 循环5次
    for _ in 0..5 {
        // 累加
        TOTAL_MS.fetch_add(ONE_DAY_MS, Ordering::Relaxed);
    }

    println!("total milliseconds: {:?}", TOTAL_MS);
}
