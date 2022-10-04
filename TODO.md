- [ ] add encoding to random string generation
- [ ] Correctness: tests + fuzzing
    - Fuzzing:
        - Generate a random set of nodes with names
        - Generate a random graph with adjacency matrix (ignore main diagonale)
        - For each node generate a random set of variables including
          the refs to other nodes from matrix
        - syntax should be valid, don't care about lexer bugs
        - if chameleon exits with 0: generated code must compile without warnings !
        - fuzz in release mode
        - purely generation based
        - exit as soon as crash is found
- [ ] Additional grammar stats ?

# Depth statistics
depth = 1 -> threads: 1 | total generated: 120.4706 MiB/s | mean size: 2
depth = 2 -> threads: 1 | total generated: 128.0000 MiB/s | mean size: 2
depth = 4 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 13
depth = 8 -> threads: 1 | total generated: 146.2857 MiB/s | mean size: 49
depth = 16 -> threads: 1 | total generated: 146.2857 MiB/s | mean size: 157
depth = 32 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 380
depth = 64 -> threads: 1 | total generated: 146.2857 MiB/s | mean size: 818
depth = 128 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 1719
depth = 256 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 3447
depth = 512 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 7041
depth = 1024 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 13963
depth = 2048 -> threads: 1 | total generated: 157.5385 MiB/s | mean size: 26847
depth = 4096 -> threads: 1 | total generated: 146.2857 MiB/s | mean size: 67743
depth = 8192 -> threads: 1 | total generated: 136.5333 MiB/s | mean size: 151369
depth = 16384 -> threads: 1 | total generated: 113.7778 MiB/s | mean size: 117464
