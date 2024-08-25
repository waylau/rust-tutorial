use test_lib_demo;

// 声明模块
mod common;

#[test]
fn it_adds_two() {
    //调用 common 中的共享函数
    common::setup();

    assert_eq!(4, test_lib_demo::add(2, 2));
}
