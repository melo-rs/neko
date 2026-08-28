use crate::io::write_all;
use core::fmt;

pub struct Writer {
    pub fd: i32,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match write_all(self.fd, s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let mut writer = $crate::fmt::Writer { fd: $crate::io::STDOUT_FILENO };
        let args = format_args!($($arg)*);

        match core::fmt::Write::write_fmt(&mut writer, args) {
            Err(_) => panic!(),
            Ok(()) => {}
        }
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        let mut writer = $crate::fmt::Writer { fd: $crate::io::STDERR_FILENO };
        let args = format_args!($($arg)*);

        match core::fmt::Write::write_fmt(&mut writer, args) {
            Err(_) => panic!(),
            Ok(()) => {}
        }
    };
}
