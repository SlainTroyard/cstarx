use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("temp")
        .unwrap();
    file.set_len(4096).unwrap();
    let len = file.seek(SeekFrom::End(0)).unwrap();

    let map = unsafe {
        libc::mmap(
            ptr::null_mut(),
            len as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if map == libc::MAP_FAILED {
        eprintln!("mmap failed");
        return;
    }

    loop {
        sleep(Duration::from_secs(1));
        let data = unsafe { slice::from_raw_parts(map as *const u8, 1024) };
        println!("{}", String::from_utf8_lossy(data));
    }

    unsafe {
        if libc::munmap(map, len as usize) == -1 {
            eprintln!("munmap failed");
        }
    }
}