#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    ffi::{CStr, c_char},
    mem::MaybeUninit,
    panic::PanicInfo,
};
use neko_rs::{
    errno::Errno,
    error::Error,
    fs::{close, fstat, openat},
    io::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, read, write_all},
    process::exit,
    retries::retry_on_eintr,
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

/// # Safety
///
/// `rsp_ptr` must point to a valid Linux x86-64 initial process stack
/// as provided to `_start`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_start(rsp_ptr: *const usize) -> ! {
    let stdout_metadata_result = retry_on_eintr(|| fstat(STDOUT_FILENO));

    let stdout_metadata = match stdout_metadata_result {
        Ok(metadata) => metadata,
        Err(errno) => {
            let _ = write_error("標準出力".as_bytes(), errno);
            terminate_after_stdout_failure();
        }
    };

    let stdout_is_file = stdout_metadata.is_file();

    // SAFETY: `rsp_ptr` is valid and points to `argc` as guaranteed by the
    // function's safety contract
    let argc = unsafe { *rsp_ptr };

    let mut buffer = [MaybeUninit::<u8>::uninit(); 4096];
    let mut had_error = false;

    if argc <= 1 {
        if stdout_is_file {
            let stdin_metadata_result = retry_on_eintr(|| fstat(STDIN_FILENO));

            let stdin_metadata = match stdin_metadata_result {
                Ok(metadata) => metadata,
                Err(errno) => {
                    let _ = write_error(b"-", errno);
                    terminate(1)
                }
            };

            if stdin_metadata.is_file()
                && stdin_metadata.st_dev == stdout_metadata.st_dev
                && stdin_metadata.st_ino == stdout_metadata.st_ino
            {
                let _ = write_error(b"-", "入力ファイルが出力ファイルです".as_bytes());
                terminate(1);
            }
        }

        let result = stream_to_stdout(STDIN_FILENO, &mut buffer);

        match result {
            Ok(()) => {}
            Err(StreamError::Read(errno)) => {
                let _ = write_error(b"-", errno);
                had_error = true
            }
            Err(StreamError::Write(errno)) => {
                let _ = write_error("書き込みエラー".as_bytes(), errno);
                terminate_after_stdout_failure();
            }
        }
    } else {
        // SAFETY: The initial stack is laid out as:
        //
        //     *const usize
        //          │
        //          ▼
        //     [ argc ][ argv[0] ][ argv[1] ] ... [ NULL ]
        //                │
        //                └── each entry is `*const c_char`
        //
        // Thus advancing past `argc` and interpreting the following words as
        // pointers gives an `*const *const c_char` pointing to `argv[0]`.
        let argv = unsafe { rsp_ptr.add(1).cast::<*const c_char>() };

        for input_index in 1..argc {
            // SAFETY: `input_index < argc`, so dereferencing this argv entry
            // is valid. Its `*const c_char` points to a NUL-terminated argument
            // string for the lifetime of the process image.
            let input = unsafe { CStr::from_ptr(*argv.add(input_index)) };

            let is_stdin = input == c"-";

            let fd = if is_stdin {
                STDIN_FILENO
            } else {
                let openat_result = retry_on_eintr(|| openat(AT_FDCWD, input, 0, 0));

                match openat_result {
                    Ok(fd) => fd,
                    Err(errno) => {
                        let _ = write_input_error(input, errno);
                        had_error = true;

                        continue;
                    }
                }
            };

            if stdout_is_file {
                let input_metadata_result = retry_on_eintr(|| fstat(fd));

                let input_metadata = match input_metadata_result {
                    Ok(metadata) => metadata,
                    Err(errno) => {
                        let _ = write_input_error(input, errno);
                        had_error = true;

                        if !is_stdin {
                            if let Err(errno) = close(fd) {
                                let _ = write_input_error(input, errno);
                            }
                        }

                        continue;
                    }
                };

                if input_metadata.is_file()
                    && input_metadata.st_dev == stdout_metadata.st_dev
                    && input_metadata.st_ino == stdout_metadata.st_ino
                {
                    let _ = write_input_error(input, "入力ファイルが出力ファイルです".as_bytes());
                    had_error = true;

                    if !is_stdin {
                        if let Err(errno) = close(fd) {
                            let _ = write_input_error(input, errno);
                        }
                    }

                    continue;
                }
            }

            let result = stream_to_stdout(fd, &mut buffer);

            match result {
                Ok(()) => {}
                Err(StreamError::Read(errno)) => {
                    let _ = write_input_error(input, errno);
                    had_error = true
                }
                Err(StreamError::Write(errno)) => {
                    let _ = write_error("書き込みエラー".as_bytes(), errno);
                    terminate_after_stdout_failure();
                }
            }

            if !is_stdin {
                if let Err(errno) = close(fd) {
                    let _ = write_input_error(input, errno);
                    had_error = true;
                }
            }
        }
    }

    terminate(if had_error { 1 } else { 0 });
}

enum StreamError {
    Read(Errno),
    Write(Errno),
}

fn stream_to_stdout(fd: i32, buffer: &mut [MaybeUninit<u8>]) -> Result<(), StreamError> {
    loop {
        let _read = match read(fd, buffer) {
            Ok(_read) => _read,
            Err(errno) if errno == Errno::EINTR => continue,
            Err(errno) => break Err(StreamError::Read(errno)),
        };

        if _read == 0 {
            break Ok(());
        }

        let initialized: &[u8] =
            // SAFETY: `read` guarantees that the first `_read` elements of 
            // `buffer` are initialized, with `_read <= buffer.len()`. 
            // Therefore this range is valid to view as an initialized `[u8]`.
            unsafe { core::slice::from_raw_parts(buffer.as_ptr().cast(), _read) };

        let result = write_all(STDOUT_FILENO, initialized);

        if let Err(errno) = result {
            break Err(StreamError::Write(errno));
        }
    }
}

fn write_error<C, E>(context: C, error: E) -> Result<(), Errno>
where
    (C, E): Error,
{
    (context, error).write_to_stderr()
}

fn write_input_error<'a, E>(input: &'a CStr, error: E) -> Result<(), Errno>
where
    (&'a CStr, E): Error,
{
    let context = if input.is_empty() { c"''" } else { input };
    (context, error).write_to_stderr()
}

fn do_terminate(status: usize, suppress_close_error: bool) -> ! {
    let mut status = status;

    if let Err(errno) = close(STDOUT_FILENO) {
        if !suppress_close_error {
            let _ = write_error("標準出力を閉じています".as_bytes(), errno);
        }

        status = 1;
    }

    if close(STDERR_FILENO).is_err() {
        status = 1;
    }

    exit(status)
}

fn terminate(status: usize) -> ! {
    do_terminate(status, false)
}

fn terminate_after_stdout_failure() -> ! {
    do_terminate(1, true)
}
