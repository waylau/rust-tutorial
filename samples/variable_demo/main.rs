/// 变量示例
fn main() {
    // 声明变量a，并初始化值1
    let a = 1;
    println!("a: {}", a);

    /*
    // 错误！不能对不可变变量a二次赋值
    a = 2;
    */

    // 声明可变变量b，并初始化值1
    let mut b = 1;
    println!("b: {b}");

    // 修改变量b的值为2
    b = 2;
    println!("b: {b}");

    /*
    // 声明未使用的变量c，并初始化值1
    let c = 1;
    // 告警！c没有任何地方用到
    */
    // 声明未使用的变量_c，并初始化值1
    let _c = 1;

    // 声明不可变变量
    let d = 1;
    println!("d: {d}");

    // 更改不可变变量的值，同时也可以更改其类型
    let d = "hi";
    println!("d: {d}");

    // 声明常量ONE_DAY_IN_SECONDS，并初始化值
    const ONE_DAY_IN_SECONDS: u64 = 60 * 60 * 24;
    println!("ONE_DAY_IN_SECONDS: {ONE_DAY_IN_SECONDS}");
}
