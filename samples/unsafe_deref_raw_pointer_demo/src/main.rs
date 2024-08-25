/// 解引用裸指针
fn main() {
    // 分配一些内存（例如，在堆上）
    let value = Box::new(5);

    // 获取裸指针
    let raw_ptr = &*value as *const i32; // 注意：这里使用 Box::leak 可能会更直接地表示“分配但忘记”

    // 解引用裸指针
    let dereferenced_value = unsafe { *raw_ptr };

    println!("Dereferenced value: {}", dereferenced_value);

    // 注意：由于我们没有手动释放内存，且没有使用 Box::leak，这里的 value 会在 main 函数结束时自动释放。
    // 如果 raw_ptr 被用来在 value 释放后访问内存，这将导致未定义行为。
}
