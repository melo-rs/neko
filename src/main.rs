#![no_std]
#![no_main]

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
    mem::MaybeUninit,
};
use neko::x86_64::SYS_EXIT;

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
        let pathname_ptr = *rsp_ptr.add(2);
        let mut file_descriptor = 0isize;

        asm!(
            "syscall",
            in("rax") 257,
            in("rdi") -100,
            in("rsi") pathname_ptr,
            in("rdx") 0,
            in("r10") 0,
            lateout("rax") file_descriptor,
            lateout("rcx") _,
            lateout("r11") _,
        );

        let mut buffer = MaybeUninit::<[u8; 4096]>::uninit();

        loop {
            let mut read = 0usize;

            asm!(
                "syscall",
                in("rax") 0,
                in("rdi") file_descriptor,
                in("rsi") buffer.as_mut_ptr() as *mut u8,
                in("rdx") 4096,
                lateout("rax") read,
                lateout("rcx") _,
                lateout("r11") _
            );

            if read == 0 {
                break
            }

            let mut offset = 0usize;

            while offset < read {
                let mut written = 0usize;

                asm!(
                    "syscall",
                    in("rax") 1,
                    in("rdi") 1,
                    in("rsi") (buffer.as_ptr() as *const u8).add(offset),
                    in("rdx") read - offset,
                    lateout("rax") written,
                    lateout("rcx") _,
                    lateout("r11") _
                );

                offset += written
            }
        }

        exit(0);
    }
}

/// Terminates the process with the given exit code.
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
