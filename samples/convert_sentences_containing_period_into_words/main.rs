/// 将包含句号的句子转为单词
fn main() {
    // 给定句子
    let sentence = String::from(
        "Entrepreneurship is jumping off a cliff and building a plane on the way down.",
    );

    println!("sentence: {sentence}");

    // 单词的起点索引
    let mut starting_index = 0;

    // 单词的结束索引
    let mut ending_index = 0;

    // 转为字符数组，并遍历字符
    for c in sentence.chars() {
        // 识别空格或者句号
        if c == ' ' || c == '.' {
            // 根据索引位置获取slice，即为单词
            let word = &sentence[starting_index..ending_index];
            println!("word: {word}");

            // starting_index位置移到下一个单词的起点位置
            starting_index = ending_index + 1;
        }

        ending_index += 1;
    }
}
