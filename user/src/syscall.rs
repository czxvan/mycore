use core::arch::asm;

pub static EXIT: usize = 93;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    syscall(64, [fd, buf as usize, len])
}

pub fn sys_exit(code: isize) -> isize{
    syscall(EXIT, [code as usize, 0, 0])
}