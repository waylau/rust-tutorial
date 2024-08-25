/// 属性宏的使用
use hello_attribute_macro::my_attribute;
use hello_attribute_macro::my_function;

#[my_attribute]
fn hello() {}

#[my_function(hi)]
fn dummy() {}

fn main() {
    hello();
    hi();
}
