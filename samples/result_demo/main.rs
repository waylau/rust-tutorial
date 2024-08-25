/// Result<T, E>
// 导入包
use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // 打开文件
    let open_file_result = File::open("hello-rust.txt");

    // 处理打开文件返回的 Result<T, E>
    match open_file_result {
        Ok(mut file) => {
            // 文件内容写入字符串
            let mut contents = String::new();
            let read_to_string_result = file.read_to_string(&mut contents);

            // 处理文件内容写入字符串返回的 Result<T, E>
            match read_to_string_result {
                Ok(s) => {
                    println!("s: {}", s);
                    println!("File opened successfully, contents: {}", contents);
                }
                Err(err) => {
                    println!("err: {}.", err);
                }
            }
        }
        Err(err) => {
            println!("Failed to open the file, err: {}.", err);
        }
    }

    // 打开文件
    let open_file_result = File::open("hello-rust.txt");

    // 处理打开文件返回的 Result<T, E>
    match open_file_result {
        Ok(mut file) => {
            // 文件内容写入字符串
            let mut contents = String::new();
            let read_to_string_result = file.read_to_string(&mut contents);

            // 处理文件内容写入字符串返回的 Result<T, E>
            match read_to_string_result {
                Ok(s) => {
                    println!("s: {}", s);
                    println!("File opened successfully, contents: {}", contents);
                }
                Err(err) => {
                    println!("err: {}.", err);
                }
            }
        }
        Err(err) => {
            // NotFound 代表尝试打开的文件并不存在
            if err.kind() == ErrorKind::NotFound {
                println!("File NotFound, err: {}.", err);
            } else {
                println!("Failed to open the file, err: {}.", err);
            }
        }
    }

}
