use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;
use std::slice;
use std::str;
use std::thread;
use std::time::Duration;

fn main() {
    let fd = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
    {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };

    let addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            4096,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            fd.as_raw_fd(),
            0,
        )
    };

    if addr == libc::MAP_FAILED {
        eprintln!("Failed to mmap");
        return;
    }

    let slice = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 4096) };

    loop {
        let offset = 1024;
        let message = b"haha, I'm fine";
        slice[offset..offset + message.len()].copy_from_slice(message);

        println!("Message written to shared memory");

        thread::sleep(Duration::from_secs(2));
    }

    unsafe {
        if libc::munmap(addr, 4096) == -1 {
            eprintln!("Failed to munmap");
        }
    }
}