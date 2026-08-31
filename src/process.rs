use crate::x86_64::SYS_EXIT_GROUP;
use core::arch::asm;

/// Terminates the calling process with the given exit status using the Linux
/// [`exit_group(2)`] system call.
///
/// [`exit_group(2)`]: https://man7.org/linux/man-pages/man2/exit_group.2.html
pub fn exit(status: usize) -> ! {
    // SAFETY: The registers follow the Linux x86-64 syscall ABI.
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_EXIT_GROUP,
            in("rdi") status,
            options(noreturn)
        )
    }
}
