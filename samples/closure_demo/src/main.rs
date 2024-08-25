/// 使用闭包简化代码
fn main() {
    // 强度
    let intensity = 10;

    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}

// 开始健身
fn just_do_it(intensity: u32) -> u32 {
    println!("just do it.....");
    intensity
}

/*
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("先做 {} 个俯卧撑!", just_do_it(intensity));
        println!("再来 {} 组卧推!", just_do_it(intensity));
    } else if random_number == 3 {
        println!("练过度了休息下吧！");
    } else {
        println!("跑步 {} 分钟!", just_do_it(intensity));
    }
}
*/

/* 
fn workout(intensity: u32, random_number: u32) {
    // 把函数 just_do_it 赋值给一个变量 do_job
    let do_job = just_do_it;

    // 通过变量 do_job 调用
    if intensity < 25 {
        println!("先做 {} 个俯卧撑!", do_job(intensity));
        println!("再来 {} 组卧推!", do_job(intensity));
    } else if random_number == 3 {
        println!("练过度了休息下吧！");
    } else {
        println!("跑步 {} 分钟!", do_job(intensity));
    }
}
*/

fn workout(intensity: u32, random_number: u32) {
    // 把函数 just_do_it 赋值给一个变量 do_job
    let do_job = || {
        println!("just do it.....");
        intensity
    };

    // 通过变量 do_job 调用
    if intensity < 25 {
        println!("先做 {} 个俯卧撑!", do_job());
        println!("再来 {} 组卧推!", do_job());
    } else if random_number == 3 {
        println!("练过度了休息下吧！");
    } else {
        println!("跑步 {} 分钟!", do_job());
    }
}
