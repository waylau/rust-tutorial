/// Result<T, E>
// 导入包
use std::fs::File;
use std::fs::Metadata;
use std::io::Error;

fn main() {
    let get_file_metadata_result = get_file_metadata();
    match get_file_metadata_result {
        Ok(metadata) => {
            dbg!("file type: {}, len: {}", metadata.file_type(), metadata.len());
        }
        Err(err) => {
            println!("err: {}", err);
        }
    }
}

// 获取文件信息
fn get_file_metadata() -> Result<Metadata, Error> {
    // 打开文件
    let open_file_result = File::open("hello-rust.txt")?;

    // 从文件获取文件信息
    let metadata = open_file_result.metadata()?;

    // 返回文件信息
    Ok(metadata)
}
