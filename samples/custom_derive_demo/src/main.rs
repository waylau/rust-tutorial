/// 自定义derive过程宏
use custom_derive_demo::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Java;

#[derive(HelloMacro)]
struct Rust;

fn main() {
    Java::hello_macro();
    Rust::hello_macro();
}