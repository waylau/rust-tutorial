/// 将单词转为句子
fn main() {
    // 创建一个字符串数组
    let word_array = ["Red", "Welcome", "Blue", "to", "waylau.com", ".", "!"];

    // 初始化可变的字符串数组
    let mut sentence: [&str; 4] = [""; 4];

    // 数组元素赋值
    sentence[0] = word_array[1];
    sentence[1] = word_array[3];
    sentence[2] = word_array[4];
    sentence[3] = word_array[6];

    dbg!("sentence: {}", sentence);

    let mut concatenation = String::from("");

    // 遍历数组
    for element in sentence {
        println!("element: {element}");

        // ne 方法用于判断字符串是否不相等
        if element.ne("!") {
            // 追加字符串
            concatenation.push_str(element);

            // 字符串拼接空格
            concatenation.push_str(" ");
        } else {
            concatenation.pop();
            // 追加字符串
            concatenation.push_str(element);
        }
    }

    println!("concatenation: {}", concatenation);
}
