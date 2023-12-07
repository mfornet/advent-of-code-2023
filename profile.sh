#!/bin/bash
set -ex

PROBLEM=$(printf "%02d" "$1")
STACK_OUTPUT="etc/out.$PROBLEM.stacks"
FLAMEGRAPH="etc/flamegraph.$PROBLEM.svg"

rm -f "$STACK_OUTPUT"
rm -f "$FLAMEGRAPH"
dtrace -c "./target/release/$PROBLEM -- --time" -o "$STACK_OUTPUT" -n "profile-997 /execname == \"$PROBLEM\"/ { @[ustack(100)] = count(); }"
stackcollapse.pl "$STACK_OUTPUT" | flamegraph.pl > "$FLAMEGRAPH"
