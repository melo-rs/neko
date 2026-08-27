#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    mem::MaybeUninit,
    panic::PanicInfo,
};
use neko::{
    io::{STDIN_FILENO, STDOUT_FILENO},
    x86_64::{AT_FDCWD, SYS_EXIT, SYS_OPENAT, SYS_READ, SYS_WRITE},
};

#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    exit(1);
}

global_asm!(
    r#"
.global _start
_start:
    mov rdi, rsp
    call do_start
"#
);

#[unsafe(no_mangle)]
pub extern "C" fn do_start(rsp_ptr: *const usize) -> ! {
    unsafe {
        let argc = *rsp_ptr;
        let mut buffer = MaybeUninit::<[u8; 4096]>::uninit();

        if argc == 1 {
            copy_to_stdout(STDIN_FILENO, &mut buffer);
        } else {
            for operand in 1..argc {
                let pathname_ptr = *rsp_ptr.add(operand + 1) as *const u8;
                let is_stdin = *pathname_ptr == b'-' && *pathname_ptr.add(1) == 0;

                let fd = if is_stdin {
                    STDIN_FILENO
                } else {
                    openat(AT_FDCWD, pathname_ptr, 0, 0)
                };

                copy_to_stdout(fd, &mut buffer);
            }
        }

        exit(0);
    }
}

fn copy_to_stdout(fd: i32, buffer: &mut MaybeUninit<[u8; 4096]>) {
    loop {
        let _read = read(fd, buffer.as_mut_ptr() as *mut u8, 4096);

        if _read < 0 {
            exit(1);
        }

        if _read == 0 {
            break;
        }

        let _read = _read as usize;
        let mut offset = 0usize;

        while offset < _read {
            let buf = unsafe { (buffer.as_ptr() as *const u8).add(offset) };
            let written = write(STDOUT_FILENO, buf, _read - offset);

            if written <= 0 {
                exit(1);
            }

            offset += written as usize
        }
    }
}

/// Terminates the current process with the given exit code.
///
/// This implementation is specific to Linux on x86-64.
fn exit(code: usize) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(noreturn)
        )
    }
}

// TODO: switch to Result<i32, Errno>
fn openat(dirfd: i32, pathname: *const u8, flags: i32, mode: u32) -> i32 {
    let file_descriptor: i32;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_OPENAT,
            in("rdi") dirfd,
            in("rsi") pathname,
            in("rdx") flags,
            in("r10") mode,
            lateout("rax") file_descriptor,
            lateout("rcx") _,
            lateout("r11") _,
        )
    };

    return file_descriptor;
}

// TODO: switch to Result<isize, Errno>
fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_READ,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    }

    return result;
}

// TODO: switch to Result<isize, Errno>
fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    let result: isize;

    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_WRITE,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") result,
            lateout("rcx") _,
            lateout("r11") _,
        )
    }

    return result;
}
