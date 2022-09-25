#!/bin/bash
set -e;

cargo build --release;

for i in *.chm;
do
    echo "Testing: $i:";
    ../../../target/release/chameleon -o /tmp/test.c --allow-cycles "$i";
    clang -o /dev/null -Wall -Wextra -Wpedantic -Werror -Wno-unused-function -c /tmp/test.c;
done

rm -f /tmp/test.c;
