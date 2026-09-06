#!/bin/sh
# Checks that neko succeeds even when applied to a file in /proc

. "${srcdir=.}/tests/init.sh"

f="/proc/cpuinfo"
test -f $f || skip_ "no $f"
neko $f >/dev/null || fail=1

exit "${fail:-0}"
