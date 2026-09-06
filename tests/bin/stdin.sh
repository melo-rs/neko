#!/bin/sh
# Checks that neko handles stdin correctly

. "${srcdir=.}/tests/init.sh"

# no arguments
echo "x" >exp || framework_failure_
echo "x" | neko >out || fail=1
compare exp out || fail=1

# dash
echo "x" >exp || framework_failure_
echo "x" | neko - >out || fail=1
compare exp out || fail=1

# dash + other files
echo "x" >fx || framework_failure_
echo "z" >fz || framework_failure_
echo "x" >exp || framework_failure_
echo "y" >>exp || framework_failure_
echo "z" >>exp || framework_failure_
echo "y" | neko fx - fz >out || fail=1
compare exp out || fail=1

# empty stdin
neko </dev/null >out || fail=1
[ ! -s out ] || fail=1

exit "${fail:-0}"
