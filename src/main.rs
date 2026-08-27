#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    mem::MaybeUninit,
    panic::PanicInfo,
};
use neko::{
    io::{STDIN_FILENO, STDOUT_FILENO, read, write},
    process::exit,
    fs::{openat, close},
    x86_64::AT_FDCWD,
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

                if !is_stdin {
                    close(fd);
                }
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
