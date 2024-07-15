/// match和if let
fn main() {
    // 访问枚举类型值
    let color = Color::Blue;
    let color_value = value_in_color(color);
    println!("color value: {}", color_value);
    let color = Color::Red;
    let color_value = value_in_color(color);
    println!("color value: {}", color_value);
    let color = Color::Green;
    let color_value = value_in_color(color);
    println!("color value: {}", color_value);

    // _ 通配符
    let u8_value = value_in_u8(1);
    println!("u8 value: {}", u8_value);
    let u8_value = value_in_u8(2);
    println!("u8 value: {}", u8_value);
    let u8_value = value_in_u8(5);
    println!("u8 value: {}", u8_value);

    // if let
    if_let_value_in_u8(1);

    if_let_value_in_u8(2);

    if_let_value_in_u8(5);
}

// 创建一个三原色枚举类型
enum Color {
    Red,
    Green,
    Blue,
}

// 将颜色枚举转为编码
fn value_in_color(color: Color) -> &'static str {
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
    }
}

// 将数字转为单词
fn value_in_u8(a: u8) -> &'static str {
    match a {
        1 => "one",
        2 => "two",
        3 => "three",
        // 通配模式
        _ => "other",
    }
}

// 将数字转为单词
fn if_let_value_in_u8(a: u8) {
    if let 1 = a {
        println!("one");
    } else if let 2 = a {
        println!("two");
    } else if let 3 = a {
        println!("three");
    } else {
        println!("other");
    }
}
