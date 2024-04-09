use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixListener;
use std::str;

fn main() {
    const SOCKET_NAME: &str = "/tmp/DemoSocket";
    const BUFFER_SIZE: usize = 128;

    // 删除可能遗留的socket文件
    std::fs::remove_file(SOCKET_NAME).ok();

    // 创建Unix域套接字
    let listener = UnixListener::bind(SOCKET_NAME).expect("bind failed");

    loop {
        // 等待连接
        println!("等待客户端连接...");
        let (mut stream, _) = listener.accept().expect("accept failed");

        let mut buffer = [0; BUFFER_SIZE];

        // 读取数据
        println!("等待客户端发送数据...");
        let mut result = 0;
        loop {
            stream.read(&mut buffer).expect("read failed");
            let data: i32 = str::from_utf8(&buffer).expect("parse failed").parse().expect("parse failed");
            if data == 0 {
                break;
            }
            result += data;
        }

        // 发送结果
        let result_str = format!("Result = {}", result);
        stream.write_all(result_str.as_bytes()).expect("write failed");
    }
}
