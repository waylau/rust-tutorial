/// 统计字符串的字符数
fn main() {
    // 定义字符串变量s；
    let s = "ILoveRust";

    // 使用 as_bytes 将 String 转为字节数数组
    let array = s.as_bytes();

    // 遍历该变量s里面的字符；
    // 每遍历一次，即统计了一次字符数；
    let mut count = 0;
    for _i in array.iter() {
        count += 1;
    }

    // 打印最终的遍历次数，即得出了该字符串的字符数。
    println!("count: {count}");
}
