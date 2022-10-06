# Reproducing the Evaluation

## Throughput
For fzero_fuzzer:    
```
git clone https://github.com/gamozolabs/fzero_fuzzer
cd fzero_fuzzer
git checkout 712a566d7acf7db6266417de2b2f9a06b5ca94e1
cargo run -- json.json bench.rs bench $DEPTH
./bench
```

For Chameleon:
```
cargo run -- --bench grammars/json.chm
```
with depths specified in the grammar file.

## Output
- fzero_fuzzer: `size-eval.rs`
- Chameleon: `main.c`

Follow the compilation instructions at the beginning of the
source files and execute the resulting binaries without any arguments.
