/// 自定义获取不到文件时的错误类型
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

// 定义FileError枚举来表示文件相关的错误
#[derive(Debug)]
enum FileError {
    NotFound(PathBuf),  // 文件找不到错误，包含文件的路径
    IoError(io::Error), // 捕获并转发标准的I/O错误
}

// 实现Error trait
impl error::Error for FileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            FileError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

// 实现Display trait以提供友好的错误消息
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::NotFound(path) => write!(f, "File not found: {}", path.display()),
            FileError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

// 示例函数，可能返回FileError
fn open_file(path: PathBuf) -> Result<(), FileError> {
    // 假设这里有一些打开文件的逻辑
    // ...

    // 模拟文件找不到的情况
    Err(FileError::NotFound(path))
}

fn main() {
    let path = PathBuf::from("hello-rust.txt");
    match open_file(path) {
        Ok(()) => println!("File opened successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
