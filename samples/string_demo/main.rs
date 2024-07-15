/// `&str`类型和String类型相互转换
fn main() {
    // 从`&str`类型生成String类型
    let s1: String = String::from("waylau.com");
    let s2: String = "waylau.com".to_string();
    println!("s1: {s1}");
    println!("s2: {s2}");

    // String类型转为`&str`类型
    let s3 = String::from("waylau.com");
    say_hello(&s3);
    say_hello(&s3[..]);
    say_hello(s3.as_str());

    // 字符串索引
    /*
    let nationality = String::from("中国人");
    let c = nationality[0]; // 错误！不可索引字符串
    */

    // 通过索引区间来访问字符串
    /* 
    let nationality = String::from("中国人");
    let s4 = &nationality[0..2]; // 错误！索引的字节没有落在字符的边界上
    println!("s4: {s4}");
    */

    // 以Unicode字符的方式遍历字符串
    let nationality = String::from("中国人");
    for c in nationality.chars() {
        println!("{}", c);
    }

    // 返回字符串的底层字节数组
    let nationality = String::from("中国人");
    for b in nationality.bytes() {
        println!("{}", b);
    }

    // 通过 \ + 字符的十六进制表示，转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    // 换行了也会保持之前的字符串格式
    // 使用\忽略换行符
    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("long_string: {long_string}");

    // 希望保持字符串的原样，不要转义
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}

fn say_hello(s: &str) {
    println!("s3: {s}");
}
