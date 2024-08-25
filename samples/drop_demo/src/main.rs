/// Drop释放资源
fn main() {
    // 使用 Drop 特征
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping data: {}", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("ccc"),
    };
    let d = CustomSmartPointer {
        data: String::from("ddd"),
    };
    println!("CustomSmartPointers created.");

    // 通过 std::mem::drop 提早丢弃值
    let e = CustomSmartPointer {
        data: String::from("eee"),
    };
    println!("CustomSmartPointer e created.");

    // 调用 std::mem::drop 显式清理
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");
}
