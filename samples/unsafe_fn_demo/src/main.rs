/// 调用不安全函数或方法
unsafe fn unsafe_fn() -> String {
    "waylau.com".to_string()
}

fn main() {
    // 在 unsafe 代码块中调用 unsafe_fn 函数
    let result = unsafe { unsafe_fn() };

    println!("result: {}", result);
}
