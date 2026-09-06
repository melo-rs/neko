#!/bin/sh
# Checks that neko handles general errors correctly

. "${srcdir=.}/tests/init.sh"

# enoent
errf="in"
echo "neko: $errf: $ENOENT" >exp_err || framework_failure_
returns_ 1 neko "$errf" >/dev/null 2>err || fail=1
compare exp_err err || fail=1

# recoverable errors
eisdirf="d"
normalf="s"
enoentf="n"
mkdir "$eisdirf" || framework_failure_
echo "normal" > $normalf || framework_failure_
echo "normal" > exp || framework_failure_
echo "neko: $eisdirf: $EISDIR" >exp_err || framework_failure_
echo "neko: $enoentf: $ENOENT" >>exp_err || framework_failure_
returns_ 1 neko $eisdirf $normalf $enoentf >out 2>err || fail=1
compare exp out || fail=1
compare exp_err err || fail=1

# read error
errf="/proc/self/mem"
if returns_ 1 dd if=$errf bs=1 count=1 status=none 2>/dev/null
then
    returns_ 1 neko "$errf" 2>err || fail=1
    err=$(cat err)
    case "$err" in
        "neko: $errf: "*) ;;
        *) fail=1 ;;
    esac
fi

# write error
if [ -w /dev/full ] && [ -c /dev/full ]
then
    echo "neko: 書き込みエラー: $ENOSPC" >exp_err || framework_failure_
    echo 1 | returns_ 1 neko >/dev/full 2>err || fail=1
    compare exp_err err || fail=1 
fi

# closed stdout
echo "neko: 標準出力: $EBADF" >exp_err || framework_failure_
echo 1 | returns_ 1 neko >&- 2>err || fail=1
compare exp_err err  || fail=1

exit "${fail:-0}"
