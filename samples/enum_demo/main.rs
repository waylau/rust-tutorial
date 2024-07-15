/// 枚举
fn main() {
    // 创建一个三原色枚举类型
    #[derive(Debug)]
    enum Color {
        Red,
        Green,
        Blue
    }

    // 访问枚举类型值
    let color = Color::Blue;
    println!("color: {:?}", color);
    let color = Color::Red;
    println!("color: {:?}", color);
    let color = Color::Green;
    println!("color: {:?}", color);

    // 枚举成员持有不同的数据类型
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Book {
        Papery(u64),
        Electronic(String),
    }

    let book = Book::Papery(9787302649113);
    println!("book: {:?}", book);
    let book = Book::Electronic(String::from("https://waylau.com/books/"));
    println!("book: {:?}", book);

    // Option 枚举类
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }

    // 定义变量时可以省略类型
    let o1 = Option::Some("Hello");
    println!("o1: {:?}", o1);

    // 定义变量时不可以省略类型
    let o2: Option<i32> = Option::None;
    println!("o2: {:?}", o2);
}
