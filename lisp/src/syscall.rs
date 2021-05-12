use alloc::string::String;
use core::mem;
use x86_64::VirtAddr;

fn fork() -> u64 {
    syscall(5, 0, 0, 0) as u64
}

fn exec(s: u64) {
    syscall(6, s, 0, 0);
}

fn set_screen_size(height: u64, width: u64) {
    syscall(11, height, width, 0);
}

fn set_screen_position(height: u64, width: u64) {
    syscall(12, height, width, 0);
}

fn open(path: String) -> usize {
    let bytes = path.as_bytes();
    let mut buffer = [0_u8; 512];
    let length = bytes.len();
    // TODO add guards
    for i in 0..length {
        buffer[i] = bytes[i]
    }
    buffer[length] = 0_u8;
    let addr = VirtAddr::from_ptr(&buffer as *const u8);
    syscall(2, addr.as_u64(), 0, 0)
}

fn push_sound(fd: u64, tone: u64, length: u64, begin: u64) {
    let sound_buffer: [u8; 24] = unsafe { mem::transmute([tone, length, begin]) };
    let addr = VirtAddr::from_ptr(&sound_buffer as *const u8);
    syscall(1, fd, addr.as_u64(), 24);
}

fn read_to_string(fd: usize, length: usize) -> String {
    let buffer = [0_u8; 512];
    let addr = VirtAddr::from_ptr(&buffer as *const u8);
    let got = syscall(0, fd as u64, addr.as_u64(), length as u64);
    let mut res = String::new();
    for i in 0..got {
        if buffer[i] == 0 {
            break;
        }
        res.push(buffer[i] as char);
    }
    res
}

fn print_buffer(buffer: &[u8], size: usize) {
    let mut index = 0_usize;
    let mut t: [u8; 256] = [0; 256];

    for c in 0..size {
        //syscall(20, index as u64, c as u64, 0);
        t[c] = buffer[c];
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), size as u64);
}

fn print(a: &String) {
    let mut t: [u8; 128] = [0; 128];
    //syscall(20, 42, 0);
    let mut index = 0_usize;

    for c in a.bytes() {
        //syscall(20, index as u64, c as u64, 0);
        t[index] = c;
        index += 1;
        if index == 128 {
            t[index - 1] = 0; // We put a guard
            break;
        }
    }
    let data_addr = VirtAddr::from_ptr(&t as *const u8);
    syscall(1, 1, data_addr.as_u64(), index as u64);
}

pub fn halt() {
    syscall(8, 0, 0, 0);
}

#[inline(never)]
pub extern "C" fn syscall(nb: u64, arg0: u64, arg1: u64, arg2: u64) -> usize {
    let res;
    unsafe {
        asm!(
            "int 80h",
            in("rax") nb, in("rdi") arg0, in("rsi") arg1, in("rdx") arg2, lateout("rax") res)
    };
    res
}
