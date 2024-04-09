use std::io::{self, Read};
use std::process::{self, Child, Command};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let child = thread::spawn(move || {
        let mut input = String::new();
        println!("Please enter a string you want to send:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();

        println!("Please enter a number you want to send:");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: i32 = number.trim().parse().expect("Failed to parse number");

        tx.send((input, number)).expect("Failed to send message");
    });

    thread::sleep(Duration::from_secs(1));

    let msg = rx.recv().expect("Failed to receive message");
    let msg_str = msg.0;
    let msg_num = msg.1;

    println!("Child process is waiting for msg: {}", msg_str);
    println!("Child process read from msg: {}", msg_num);

    child.join().expect("Failed to join thread");
}