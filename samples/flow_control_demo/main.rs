/// 流程控制示例
// main函数是程序入口
fn main() {
    // 分支控制
    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("number: {number}");

    // 处理多重条件
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // while循环
    let mut number = 1;
    while number != 4 {
        println!("number: {number}");
        number += 1;
    }

    // for循环
    let array = [10, 20, 30, 40, 50];
    for number in array.iter() {
        println!("number: {number}");
    }

    // loop循环
    let array = ['R', 'U', 'S', 'T'];
    let mut i = 0;
    loop {
        let ch = array[i];
        if ch == 'T' {
            break;
        }
        println!("ch: {ch}");
        i += 1;
    }
}
