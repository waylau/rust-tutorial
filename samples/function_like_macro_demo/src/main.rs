/// 实战：类函数宏的使用
use hello_function_like_macro::print_hello;
use hello_function_like_macro::print_message;

fn main() {
    print_hello!();

    print_message!(SELECT * FROM posts WHERE id=1);
}
