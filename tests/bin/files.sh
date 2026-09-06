#!/bin/bash
# Checks that neko handles regular files correctly

. "${srcdir=.}/tests/init.sh"

# file
echo "x" >f || framework_failure_
echo "x" >exp || framework_failure_
neko f >out || fail=1
compare exp out || fail=1

# multiple files
echo "x" >fx || framework_failure_
echo "y" >fy || framework_failure_
echo "x" >exp || framework_failure_
echo "y" >>exp || framework_failure_
neko fx fy >out || fail=1
compare exp out || fail=1

# empty file
: >f || framework_failure_
neko f >out || fail=1
[ ! -s out ] || fail=1

# file larger than the read buffer
dd if=/dev/zero of=in bs=8193 count=1 status=none || framework_failure_
neko in >out || fail=1

exit "${fail:-0}"
