/// 全局ID生成器
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    // 检查两次溢出，否则直接加一可能导致溢出
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory ids overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    let next_id = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if next_id > MAX_ID {
        panic!("Factory ids overflowed");
    }
    next_id
}

fn main() {
    // 遍历10次
    for _ in 0..10 {
        println!("generate id: {}", generate_id());
    }
}
