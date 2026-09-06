#!/bin/sh
# Checks that neko operates correctly when its stdin and stdout are the same

. "${srcdir=.}/tests/init.sh"

# file = stdout
echo x >out || framework_failure_
echo x >exp || framework_failure_
# shellcheck disable=SC2094
returns_ 1 neko out >>out 2>err || fail=1
compare exp out || fail=1

# from POSIX spec for 'cat'
echo x >doc || framework_failure_
echo y >doc.end || framework_failure_
# shellcheck disable=SC2094
# TODO: exit with 0 for empty files
returns_ 1 neko doc doc.end >doc 2>/dev/null || fail=1
compare doc doc.end || fail=1

# stdin = stdout
echo x >in || framework_failure_
echo x >exp || framework_failure_
# shellcheck disable=SC2094
returns_ 1 neko <in >>in 2>err || fail=1
compare exp in || fail=1

exit "${fail:-0}"
