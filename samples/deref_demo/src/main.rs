/// Deref解引用
fn main() {
    // 追踪指针的值
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用解引用运算符来跟踪所引用的值

    // 像引用一样使用`Box<T>`
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Box<T> 上使用的解引用运算符

    // 自定义智能指针
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现Deref特征
    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // MyBox<T> 上使用的解引用运算符
}
