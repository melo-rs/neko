/// A linux error number
///
/// See [`errno(3)`] for the list of error codes and their meanings.
///
/// [`errno(3)`]: https://man7.org/linux/man-pages/man3/errno.3.html
#[derive(PartialEq, Eq, Debug)]
pub struct Errno(pub u16);

pub struct Metadata<'a> {
    pub reason: &'a [u8],
    pub is_unknown: bool,
}

macro_rules! constant {
    ($reason:expr) => {
        Metadata {
            reason: $reason.as_bytes(),
            is_unknown: false,
        }
    };
}

macro_rules! unknown {
    ($reason:expr) => {
        Metadata {
            reason: $reason.as_bytes(),
            is_unknown: true,
        }
    };
}

impl Errno {
    pub const EPERM: Errno = Errno(1);
    pub const ENOENT: Errno = Errno(2);
    pub const EINTR: Errno = Errno(4);
    pub const EIO: Errno = Errno(5);
    pub const EBADF: Errno = Errno(9);
    pub const EAGAIN: Errno = Errno(11);
    pub const EACCES: Errno = Errno(13);
    pub const EFAULT: Errno = Errno(14);
    pub const EBUSY: Errno = Errno(16);
    pub const EEXIST: Errno = Errno(17);
    pub const ENOTDIR: Errno = Errno(20);
    pub const EISDIR: Errno = Errno(21);
    pub const EINVAL: Errno = Errno(22);
    pub const ENFILE: Errno = Errno(23);
    pub const EMFILE: Errno = Errno(24);
    pub const ENOSPC: Errno = Errno(28);
    pub const EPIPE: Errno = Errno(32);

    pub const fn metadata<'a>(&self) -> Metadata<'a> {
        match self.0 {
            1 => constant!("許可されていない操作です"),
            2 => constant!("そのようなファイルやディレクトリはありません"),
            4 => constant!("システムコール割り込み"),
            5 => constant!("入力/出力エラーです"),
            9 => constant!("不正なファイル記述子です"),
            11 => constant!("リソースが一時的に利用できません"),
            13 => constant!("許可がありません"),
            14 => constant!("不正なアドレスです"),
            16 => constant!("デバイスもしくはリソースがビジー状態です"),
            17 => constant!("ファイルが存在します"),
            20 => constant!("ディレクトリではありません"),
            21 => constant!("ディレクトリです"),
            22 => constant!("無効な引数です"),
            23 => constant!("システム中のファイルを開きすぎです"),
            24 => constant!("ファイルを開きすぎです"),
            28 => constant!("デバイスに空き領域がありません"),
            32 => constant!("壊れたパイプです"),
            _ => unknown!("不明なエラー "),
        }
    }
}
