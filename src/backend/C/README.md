# C Backend

This backend takes a chameleon grammar and outputs a .c file
that can then be compiled into an executable or an AFL++ custom mutator, etc.

## Requirements
This backend assumes that the target architecture is 64-bit.

## Code generation
- entry function = `$PREFIX_generate(unsigned char*, size_t) -> size_t`
- `Container` own function
- oneof scheduling:
    - for round-robin: keep map id->idx for each oneof and define `oneof_round_robin(size_t id, size_t len)`
- RNG: (xorshift)
    - `$PREFIX_seed(size_t)`
- Numberset: functions with return type (u)int<size>_t
    - first randomly determine range
    - determine random number in range
