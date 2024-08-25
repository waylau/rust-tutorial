/// 使用`Box::leak`
fn main() {
    static mut DATA: *const i32 = std::ptr::null();

    unsafe {
        // 只有第一次运行的时候才初始化 DATA
        if DATA.is_null() {
            let boxed_data = Box::new(42);
            DATA = Box::leak(boxed_data);
        }

        println!("Global data: {}", *DATA);
    }
}
