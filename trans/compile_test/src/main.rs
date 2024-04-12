use std::ffi::CString;
use std::ptr;
use std::str;
use std::time::Duration;
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::process::Command;

extern crate libc;

use libc::{c_int, c_void, close, epoll_create1, epoll_ctl, epoll_event, EPOLLIN, EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLL_CTL_MOD, ftruncate, fork, shm_open, O_CREAT, O_RDWR, PROT_READ, PROT_WRITE, MAP_SHARED, shm_unlink};

const SHM_SIZE: usize = 1024;
const SHM_NAME: &str = "shm_example";

#[repr(C)]
struct Message {
    written: i32,
    text: [i8; SHM_SIZE - std::mem::size_of::<i32>()],
}

fn main() {
    unsafe {
        // 创建/打开共享内存对象
        let shm_fd = shm_open(CString::new(SHM_NAME).unwrap().as_ptr(), O_CREAT | O_RDWR, 0o666);
        if shm_fd < 0 {
            eprintln!("Failed to open shared memory object");
            return;
        }

        // 设置共享内存大小
        if ftruncate(shm_fd, SHM_SIZE as i32) < 0 {
            eprintln!("Failed to set shared memory size");
            close(shm_fd);
            return;
        }

        // 将共享内存映射到进程地址空间
        let shm_ptr: *mut Message = mmap(ptr::null_mut(), SHM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, shm_fd, 0) as *mut Message;
        if shm_ptr.is_null() {
            eprintln!("Failed to map shared memory");
            close(shm_fd);
            return;
        }

        // 初始化共享内存
        (*shm_ptr).written = 0;

        // 创建子进程
        match fork() {
            -1 => {
                eprintln!("Failed to fork");
                close(shm_fd);
                return;
            }
            0 => {
                // 子进程 - 客户端
                client(shm_ptr);
            }
            _ => {
                // 父进程 - 服务器
                server(shm_ptr);
            }
        }

        // 断开共享内存映射
        munmap(shm_ptr as *mut c_void, SHM_SIZE);

        // 删除共享内存对象
        shm_unlink(CString::new(SHM_NAME).unwrap().as_ptr());

        // 关闭文件描述符
        close(shm_fd);
    }
}

fn client(shm_ptr: *mut Message) {
    let mut buffer = [0; 1024];

    loop {
        // 等待共享内存可写
        while (*shm_ptr).written != 0 {
            std::thread::sleep(Duration::from_secs(1));
        }

        print!("Enter some text: ");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read(&mut buffer);

        // 写入共享内存
        let text = str::from_utf8(&buffer).unwrap().trim_end_matches('\n');
        let bytes = text.as_bytes();
        let len = bytes.len().min((*shm_ptr).text.len());
        (*shm_ptr).text[..len].copy_from_slice(&bytes[..len]);
        (*shm_ptr).written = 1;
    }
}

fn server(shm_ptr: *mut Message) {
    let mut epoll_fd = epoll_create1(0);
    if epoll_fd < 0 {
        eprintln!("Failed to create epoll instance");
        return;
    }

    let mut ev = epoll_event {
        events: EPOLLIN,
        u64: 0,
    };

    // 添加标准输入到epoll实例
    if epoll_ctl(epoll_fd, EPOLL_CTL_ADD, libc::STDIN_FILENO, &mut ev) < 0 {
        eprintln!("Failed to add stdin to epoll");
        close(epoll_fd);
        return;
    }

    loop {
        let mut events = [epoll_event { events: 0, u64: 0 }; 1];
        let nfds = epoll_wait(epoll_fd, &mut events, 1, 500);

        if nfds < 0 {
            eprintln!("Failed to wait for epoll events");
            close(epoll_fd);
            return;
        }

        if nfds > 0 {
            // 读取共享内存
            if (*shm_ptr).written != 0 {
                let text = str::from_utf8(&(*shm_ptr).text).unwrap();
                println!("Server received: {}", text);
                (*shm_ptr).written = 0;
            }
        }
    }
}
