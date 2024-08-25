/// 简化错误处理
// 导入包
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // unwrap和expect
    /*
    // 打开文件
    let open_file_result = File::open("hello-rust.txt");

    // unwrap处理Err
    open_file_result.unwrap();

    // 打开文件
    let open_file_result = File::open("hello-rust.txt");

    // expect处理Err
    open_file_result.expect("Failed to open the file.");
    */
    // 传播错误
    let get_file_content_result = get_file_content();
    match get_file_content_result {
        Ok(content) => {
            println!("content: {}", content);
        }
        Err(err) => {
            println!("err: {}", err);
        }
    }
}

// 获取文件内容
/*
fn get_file_content() -> Result<String, io::Error> {
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
                Ok(_) => Ok(contents),
                Err(err) => Err(err),
            }
        }
        Err(err) => Err(err),
    }
}
*/
// `?`运算符简化传播错误
fn get_file_content() -> Result<String, io::Error> {
    // 打开文件
    let mut open_file_result = File::open("hello-rust.txt")?;

    // 文件内容写入字符串
    let mut contents = String::new();
    open_file_result.read_to_string(&mut contents)?;

    // 处理文件内容写入字符串返回的 Result<T, E>
    Ok(contents)
}
