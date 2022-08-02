#!/bin/bash

export AFL_TMPDIR="/tmp"
export AFL_NO_AUTODICT=1
export AFL_AUTORESUME=1
export AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=1
export AFL_SKIP_CPUFREQ=1
cargo afl fuzz -Mafl -F artifacts/outputs/queue -F artifacts/outputs/signaled -m 1024 -i corpus -o artifacts -- ./target/debug/chameleon-fuzz
