/// 实战：操作字符串
fn main() {
    // 追加
    // 必须加 mut 关键字
    let mut s = String::from("Welcome to ");

    s.push_str("waylau.com");
    println!("push_str() -> {}", s);

    s.push('!');
    println!("push() -> {}", s);

    // 插入
    // 必须加 mut 关键字
    let mut s = String::from("Welcome");
    s.insert(7, '!');
    println!("insert() -> {}", s);
    s.insert_str(8, " waylau.com");
    println!("insert_str() -> {}", s);

    // 替换
    let s = String::from("Welcome to waylau.com! Welcome to China!");
    let new_s = s.replace("Welcome", "WELCOME");
    println!("replace() -> {}", new_s);

    let s = String::from("Welcome to waylau.com! Welcome to China!");
    let new_s = s.replacen("Welcome", "WELCOME", 1);
    println!("replacen() -> {}", new_s);

    let mut s = String::from("Welcome to waylau.com! Welcome to China!");
    // 不会返回新的字符串
    s.replace_range(0..7, "WELCOME");
    println!("replace_range() -> {}", s);

    // pop 删除并返回字符串的最后一个字符
    let mut s = String::from("跟老卫学Rust!");
    let p = s.pop();
    dbg!("{}", p);
    let p = s.pop();
    dbg!("{}", p);
    let p = s.pop();
    dbg!("{}", p);
    let p = s.pop();
    dbg!("{}", p);
    let p = s.pop();
    dbg!("{}", p);
    let p = s.pop();
    dbg!("{}", p);
    println!("s: {}", s);

    // remove 删除并返回字符串中指定位置的字符
    let mut s = String::from("跟老卫学Rust!");
    let r = s.remove(0);
    println!("remove() -> {}", r);
    let r = s.remove(0);
    println!("remove() -> {}", r);
    let r = s.remove(0);
    println!("remove() -> {}", r);
    println!("s: {}", s);

    // truncate 删除字符串中从指定位置开始到结尾的全部字符
    let mut s = String::from("跟老卫学Rust!");
    // 注意，一个汉字占3个字节
    s.truncate(12);
    println!("s: {}", s);

    // clear 清空字符串
    let mut s = String::from("跟老卫学Rust!");
    s.clear();
    println!("s len: {}", s.len());

    // 使用 + 或者 += 连接字符串
    let s1 = String::from("跟老卫");
    let s2 = String::from("学");
    // 使用 + 时， 必须传递切片引用类型
    let s3 = s1 + &s2;
    let mut s4 = s3 + "Rust";
    s4 += "!";
    println!("s4: {}", s4);
}
