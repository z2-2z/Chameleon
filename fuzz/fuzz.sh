#!/bin/bash
set -e;

if [[ -z "$1" ]];
then
    echo "Usage: $0 <fuzz target>";
    echo "Where <fuzz target> can be:";
    cargo fuzz list;
    exit 1;
fi

target="$1";

cargo fuzz list | grep -q "$target" || (echo "Invalid fuzz target" && exit 1);

dictflag="";
if [[ -f "$target.dict" ]];
then
    dictflag="-dict=$target.dict";
fi

cargo fuzz run "$target" -- -only_ascii=1 -max_len=128 -timeout=10 "$dictflag";

# coverage report:
# cargo fuzz coverage <target>
# llvm-cov show -instr-profile=coverage/<target>/coverage.profdata ./target/x86_64-unknown-linux-gnu/release/<target> -show-line-counts-or-regions -show-instantiations -format=html -output-dir=coverage/<target>
