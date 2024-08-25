/// 结构中的生命周期
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
