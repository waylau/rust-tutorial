/// 实战：有效括号序列
fn main() {
    // 有效
    let parenthes = String::from("{[]}");
    println!("{}", is_valid(parenthes));

    // 无效
    let parenthes = String::from("{[]}}");
    println!("{}", is_valid(parenthes));
}

fn is_valid(s: String) -> bool {
    // 字符数组转为动态数组
    let chars: Vec<char> = s.chars().collect();
    if chars.len() == 0 {
        return true;
    }

    // 动态数组当作栈
    let mut stack: Vec<char> = Vec::new();
    for i in 0..chars.len() {
        // 遇到左括号则入栈，其他字符则出栈
        // 出栈时，判断当前字符是否等于栈顶字符，如果不相等则证明括号非法。
        if chars[i] == '(' {
            stack.push(')');
        } else if chars[i] == '[' {
            stack.push(']');
        } else if chars[i] == '{' {
            stack.push('}');
        } else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
            return false;
        }
    }

    return stack.is_empty();
}
