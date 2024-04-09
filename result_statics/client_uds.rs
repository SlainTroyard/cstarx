use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::str;

fn main() {
    let socket_path = "/tmp/DemoSocket";
    let mut stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };

    loop {
        print!("Enter number to send to server: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Invalid number: {}", e);
                continue;
            }
        };

        if number == 0 {
            break;
        }

        stream.write_all(&number.to_le_bytes()).unwrap();
        println!("Sent number {}", number);

        let mut buffer = [0; 128];
        stream.read_exact(&mut buffer).unwrap();
        let result = str::from_utf8(&buffer).unwrap();
        println!("Received result: {}", result);
    }

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}