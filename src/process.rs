use crate::x86_64::SYS_EXIT;
use core::arch::asm;

/// Terminates the calling process with the given exit status using the Linux
/// [`exit(2)`] system call.
///
/// [`exit(2)`]: https://man7.org/linux/man-pages/man2/_exit.2.html
pub fn exit(code: usize) -> ! {
    // SAFETY: The registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") code,
            options(noreturn)
        )
    }
}
