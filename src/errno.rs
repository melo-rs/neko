/// A Linux error number.
///
/// See [`errno(3)`] for the list of error codes and their meanings.
///
/// [`errno(3)`]: https://man7.org/linux/man-pages/man3/errno.3.html
#[derive(PartialEq, Eq, Debug)]
pub struct Errno(pub u16);

macro_rules! errno_codes {
    (
        $(
            ($name:ident, $number:literal, $cause:literal),
        )+
    ) => {
        impl Errno {
            $(
                pub const $name: Self = Self($number);
            )+

            pub const fn cause(&self) -> &'static [u8] {
                match self.0 {
                    $(
                        $number => $cause.as_bytes(),
                    )+
                    _ => "不明なエラー".as_bytes(),
                }
            }

            pub const fn is_unknown(&self) -> bool {
                match self.0 {
                    $(
                        $number => false,
                    )+
                    _ => true,
                }
            }
        }
    };
}

// TODO: add `EXFULL` and `EHWPOISON`
errno_codes! {
    (EPERM, 1, "許可されていない操作です"),
    (ENOENT, 2, "そのようなファイルやディレクトリはありません"),
    (ESRCH, 3, "そのようなプロセスはありません"),
    (EINTR, 4, "システムコール割り込み"),
    (EIO, 5, "入力/出力エラーです"),
    (ENXIO, 6, "そのようなデバイスやアドレスはありません"),
    (E2BIG, 7, "引数リストが長すぎます"),
    (ENOEXEC, 8, "実行形式エラー"),
    (EBADF, 9, "不正なファイル記述子です"),
    (ECHILD, 10, "子プロセスがありません"),
    (EAGAIN, 11, "リソースが一時的に利用できません"),
    (ENOMEM, 12, "メモリを確保できません"),
    (EACCES, 13, "許可がありません"),
    (EFAULT, 14, "不正なアドレスです"),
    (ENOTBLK, 15, "ブロックデバイスが必要です"),
    (EBUSY, 16, "デバイスもしくはリソースがビジー状態です"),
    (EEXIST, 17, "ファイルが存在します"),
    (EXDEV, 18, "無効なクロスデバイスリンクです"),
    (ENODEV, 19, "そのようなデバイスはありません"),
    (ENOTDIR, 20, "ディレクトリではありません"),
    (EISDIR, 21, "ディレクトリです"),
    (EINVAL, 22, "無効な引数です"),
    (ENFILE, 23, "システム中のファイルを開きすぎです"),
    (EMFILE, 24, "ファイルを開きすぎです"),
    (ENOTTY, 25, "デバイスに対する不適切なioctlです"),
    (ETXTBSY, 26, "テキストファイルがビジー状態です"),
    (EFBIG, 27, "ファイルが大きすぎます"),
    (ENOSPC, 28, "デバイスに空き領域がありません"),
    (ESPIPE, 29, "不正なシークです"),
    (EROFS, 30, "読み込み専用ファイルシステムです"),
    (EMLINK, 31, "リンクが多すぎます"),
    (EPIPE, 32, "壊れたパイプです"),
    (EDOM, 33, "数値の引数はドメイン外です"),
    (ERANGE, 34, "計算結果は範囲外の値です"),
    (EDEADLK, 35, "リソースのデッドロック回避"),
    (ENAMETOOLONG, 36, "ファイル名が長すぎます"),
    (ENOLCK, 37, "ロックが利用できません"),
    (ENOSYS, 38, "関数は実装されていません"),
    (ENOTEMPTY, 39, "ディレクトリは空ではありません"),
    (ELOOP, 40, "シンボリックリンクの階層が多すぎます"),
    (ENOMSG, 42, "要求した形式のメッセージはありません"),
    (EIDRM, 43, "識別子を除去しました"),
    (ECHRNG, 44, "チャンネル番号が範囲外です"),
    (EL2NSYNC, 45, "レベル2は同期していません"),
    (EL3HLT, 46, "レベル3停止"),
    (EL3RST, 47, "レベル3はリセットしました"),
    (ELNRNG, 48, "リンク数が範囲外です"),
    (EUNATCH, 49, "プロトコルドライバがアタッチされていません"),
    (ENOCSI, 50, "CSI 構造が利用できません"),
    (EL2HLT, 51, "レベル2停止"),
    (EBADE, 52, "無効な交換です"),
    (EBADR, 53, "無効なリクエスト記述子です"),
    (ENOANO, 55, "アノードがありません"),
    (EBADRQC, 56, "無効なリクエストコードです"),
    (EBADSLT, 57, "無効なスロットです"),
    (EBFONT, 59, "不正なフォントファイル形式です"),
    (ENOSTR, 60, "デバイスはストリーム型ではありません"),
    (ENODATA, 61, "利用可能なデータがありません"),
    (ETIME, 62, "タイマが破棄されました"),
    (ENOSR, 63, "ストリームリソース外です"),
    (ENONET, 64, "マシンはネットワークにつながっていません"),
    (ENOPKG, 65, "パッケージはインストールされていません"),
    (EREMOTE, 66, "オブジェクトはリモートにあります"),
    (ENOLINK, 67, "リンクが切れています"),
    (EADV, 68, "Advertiseエラー"),
    (ESRMNT, 69, "Srmount エラー"),
    (ECOMM, 70, "送信中の通信エラー"),
    (EPROTO, 71, "プロトコルエラー"),
    (EMULTIHOP, 72, "多重ホップが企てられました"),
    (EDOTDOT, 73, "RFS特定エラー"),
    (EBADMSG, 74, "不正なメッセージです"),
    (EOVERFLOW, 75, "定義されたデータ型に対して値が大きすぎます"),
    (ENOTUNIQ, 76, "名前がネットワーク上で重複しています"),
    (EBADFD, 77, "ファイル記述子が不正の状態にあります"),
    (EREMCHG, 78, "遠隔アドレスが変更されました"),
    (ELIBACC, 79, "必要な共有ライブラリにアクセスできません"),
    (ELIBBAD, 80, "壊れた共有ライブラリにアクセスしています"),
    (ELIBSCN, 81, "a.out 中の .lib セクションが壊れています"),
    (ELIBMAX, 82, "あまりに多過ぎる共有ライブラリへリンクしようとしています"),
    (ELIBEXEC, 83, "共有ライブラリは直接実行できません"),
    (EILSEQ, 84, "無効または不完全なマルチバイトまたはワイド文字です"),
    (ERESTART, 85, "割り込まれたシステムコールは再スタートさせるべきです"),
    (ESTRPIPE, 86, "ストリームパイプエラー"),
    (EUSERS, 87, "ユーザが多すぎます"),
    (ENOTSOCK, 88, "ソケットでないものにソケット操作をしています"),
    (EDESTADDRREQ, 89, "送信先アドレスが必要です"),
    (EMSGSIZE, 90, "メッセージが長すぎます"),
    (EPROTOTYPE, 91, "ソケットに対し間違ったプロトコルの形式です"),
    (ENOPROTOOPT, 92, "プロトコルは利用できません"),
    (EPROTONOSUPPORT, 93, "プロトコルはサポートされていません"),
    (ESOCKTNOSUPPORT, 94, "ソケット形式はサポートしていません"),
    (EOPNOTSUPP, 95, "サポートされていない操作です"),
    (EPFNOSUPPORT, 96, "プロトコルファミリはサポートしていません"),
    (EAFNOSUPPORT, 97, "アドレスファミリはプロトコルによってサポートされていません"),
    (EADDRINUSE, 98, "アドレスは既に使用中です"),
    (EADDRNOTAVAIL, 99, "要求アドレスに割り当てられません"),
    (ENETDOWN, 100, "ネットワークが落ちています"),
    (ENETUNREACH, 101, "ネットワークに届きません"),
    (ENETRESET, 102, "リセット中ネットワークの接続が切れました"),
    (ECONNABORTED, 103, "ソフトウェアが接続を中断しました"),
    (ECONNRESET, 104, "接続が相手からリセットされました"),
    (ENOBUFS, 105, "利用可能な空きバッファがありません"),
    (EISCONN, 106, "通信端点が既に接続されています"),
    (ENOTCONN, 107, "通信端点が接続されていません"),
    (ESHUTDOWN, 108, "通信端点のシャットダウン後は送信できません"),
    (ETOOMANYREFS, 109, "参照が多すぎます: 接続できません"),
    (ETIMEDOUT, 110, "接続がタイムアウトしました"),
    (ECONNREFUSED, 111, "接続を拒否されました"),
    (EHOSTDOWN, 112, "ホストが落ちています"),
    (EHOSTUNREACH, 113, "ホストへの経路がありません"),
    (EALREADY, 114, "操作はすでに処理中です"),
    (EINPROGRESS, 115, "現在処理中の操作です"),
    (ESTALE, 116, "古いファイルハンドルです"),
    (EUCLEAN, 117, "構造体を内容消去する必要があります"),
    (ENOTNAM, 118, "XENIX の名前付きファイルではありません"),
    (ENAVAIL, 119, "XENIX セマフォが利用できません"),
    (EISNAM, 120, "名前付きファイルです"),
    (EREMOTEIO, 121, "遠隔I/Oエラーです"),
    (EDQUOT, 122, "ディスク使用量制限を超過しました"),
    (ENOMEDIUM, 123, "メディアが見つかりません"),
    (EMEDIUMTYPE, 124, "不正なメディア形式です"),
    (ECANCELED, 125, "操作は中断されました"),
    (ENOKEY, 126, "要求されたキーが利用できません"),
    (EKEYEXPIRED, 127, "キーが期限切れです"),
    (EKEYREVOKED, 128, "キーが破棄されています"),
    (EKEYREJECTED, 129, "キーがサービスによって拒否されました"),
    (EOWNERDEAD, 130, "所有者が無くなりました"),
    (ENOTRECOVERABLE, 131, "状態の復帰が出来ません"),
    (ERFKILL, 132, "RF-kill のため操作は不可能です"),
}

impl Errno {
    pub const EWOULDBLOCK: Self = Self::EAGAIN;
    pub const EDEADLOCK: Self = Self::EDEADLK;
    pub const ENOTSUP: Self = Self::EOPNOTSUPP;
}
