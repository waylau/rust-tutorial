/// 使用静态变量

// 一天的毫秒数
static ONE_DAY_MS: usize = 24 * 60 * 60 * 1000;

// 毫秒数总计
static mut TOTAL_MS: usize = 0;

fn main() {
    // 在访问和修改静态变量的地方，需要使用unsafe语句块
    unsafe {
        // 循环5次
        for _ in 0..5 {
            TOTAL_MS += ONE_DAY_MS;
        }

        println!("total milliseconds: {TOTAL_MS}");
    }
}
