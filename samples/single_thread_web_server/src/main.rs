use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // 在地址 127.0.0.1:8080 上监听传入的 TCP 流
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        // 处理请求
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // 读取HTTP请求的第一行
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // 解析请求并构造响应
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // 读取文件内容
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    // 发送响应
    stream.write_all(response.as_bytes()).unwrap();

    // 关闭连接
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
