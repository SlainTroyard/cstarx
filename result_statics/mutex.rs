use std::thread;
use std::sync::{Mutex, Arc};
use std::time::Duration;

fn main() {
    let mutex = Arc::new(Mutex::new(()));

    let handle1 = thread::spawn({
        let mutex = Arc::clone(&mutex);
        move || {
            loop {
                let _guard = mutex.lock().unwrap();
                for _ in 0..10 {
                    println!("Hello world");
                    thread::sleep(Duration::from_secs(1));
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    let handle2 = thread::spawn({
        let mutex = Arc::clone(&mutex);
        move || {
            loop {
                let _guard = mutex.lock().unwrap();
                for _ in 0..10 {
                    println!("Good morning");
                    thread::sleep(Duration::from_secs(1));
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("The thread is over, process is over too.");
}