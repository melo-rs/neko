#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::io::write_vectored(
            STDERR_FILENO,
            &mut [b"\n".to_write_vector()],
        )
    };

    ($($arg:expr),+ $(,)?) => {
        $crate::io::write_vectored(
            STDERR_FILENO,
            &mut [
                $(
                    $arg.to_write_vector(),
                )+
                b"\n".to_write_vector(),
            ],
        )
    };
}
