/// 元组与结构体
fn main() {
    // 创建一个关于书本信息的元组，包含书名、作者、单价、库存
    let book_info: (&str, &str, f64, i32) = ("跟老卫学Rust!", "老卫", 39.3, 112);

    // 通过元组的索引获取元组中的值
    let book_name = book_info.0;
    let author = book_info.1;
    let price = book_info.2;
    let inventory = book_info.3;

    println!(
        "book_info, book_name:{book_name}, author:{author}, price:{price}, inventory:{inventory}"
    );

    // 创建 BookInfo 结构体的实例
    let book_info = BookInfo {
        book_name: String::from("跟老卫学Rust!"),
        author: String::from("老卫"),
        price: 39.3,
        inventory: 112,
    };

    // 访问结构体字段
    dbg! {book_info.book_name};
    dbg! {book_info.author};
    dbg! {book_info.price};
    dbg! {book_info.inventory};

    // 修改结构体字段
    // 必须要将结构体实例声明为 mut
    let mut book_info = BookInfo {
        book_name: String::from("跟老卫学Rust!"),
        author: String::from("老卫"),
        price: 39.3,
        inventory: 112,
    };

    // 修改结构体字段
    book_info.price = 99.8;

    dbg! {book_info.book_name};
    dbg! {book_info.author};
    dbg! {book_info.price};
    dbg! {book_info.inventory};

    // 结构体更新语法
    let book_info = BookInfo {
        book_name: String::from("跟老卫学Rust!"),
        author: String::from("老卫"),
        price: 39.3,
        inventory: 112,
    };

    let book_info2 = BookInfo {
        price: 99.3,
        // 结构体更新语法。没有显式声明的字段，全部从 book_info 中自动获取
        ..book_info
    };

    dbg! {book_info2.book_name};
    dbg! {book_info2.author};
    dbg! {book_info2.price};
    dbg! {book_info2.inventory};

    // 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);

    dbg! {black.0};
    dbg! {black.1};
    dbg! {black.2};
    dbg! {origin.0};
    dbg! {origin.1};
    dbg! {origin.2};
}

// 以下结构体定义了书本信息
struct BookInfo {
    book_name: String,
    author: String,
    price: f64,
    inventory: i32,
}
