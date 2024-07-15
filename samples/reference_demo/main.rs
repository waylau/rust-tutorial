/// 引用与借用
fn main() {
    // 引用与解引用
    let x = 5;
    let y = &x; // 引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用

    /* 
    assert_eq!(5, y); // 错误！无法比较整数类型和引用类型
    */

    // 不可变引用
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 修改不可变引用
    /* 
    let s = String::from("hello");
    change_reference(&s); // 尝试修改借用的变量
    */

    // 修改可变引用
    let mut s = String::from("hello"); // 可变变量
    change_mutable_reference(&mut s); // 尝试修改借用的变量

    // 悬垂引用
    /* 
    let reference_to_nothing = dangle();
    */

    // 避免悬垂引用
    let reference_to_string = avoid_dangle();
    println!("reference_to_string: {reference_to_string}");
}

fn calculate_length(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
  // 所以什么也不会发生

/* 
fn change_reference(some_string: &String) {
    some_string.push_str(", world"); // 错误！不能修改借用的变量
}
*/

fn change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world"); // 可变引用允许修改借用的变量
}

/* 
fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
*/

fn avoid_dangle() -> String { // avoid_dangle 返回一个字符串

    let s = String::from("hello"); // s 是一个新字符串

    s // 返回字符串 s
}
