#!/bin/bash
set -e;

cargo build;

for i in *.chm;
do
    echo "Testing: $i:";
    ../../../target/debug/chameleon -o /tmp/test.c "$i";
    clang -o /dev/null -Wall -Wextra -Wpedantic -Werror -Wno-unused-function -c /tmp/test.c;
done

rm -f /tmp/test.c;
