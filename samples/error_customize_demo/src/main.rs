/// 自定义错误类型
use std::error;
use std::fmt;

// 定义一个枚举来表示可能的错误情况
#[derive(Debug)]
enum MyError {
    NotFound,
    PermissionDenied,
    // 可以根据需要添加更多错误变体
}

// 为MyError实现Error特征
impl error::Error for MyError {}

// 为MyError实现Display特征（可选，但推荐），以便在需要时输出友好的错误信息
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::NotFound => write!(f, "not found"),
            MyError::PermissionDenied => write!(f, "permission denied"),
            // 添加更多错误变体的处理
        }
    }
}

// 示例函数，可能返回MyError
fn my_function() -> Result<(), MyError> {
    // 假设这里有一些逻辑判断
    // ...

    // 根据逻辑条件返回不同的错误
    Err(MyError::NotFound)
    // 或者
    // Err(MyError::PermissionDenied)
    // 或者在没有错误时
    // Ok(())
}

fn main() {
    match my_function() {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
}
