# Chameleon

<span>
    <img align="right" src="cutie.png" width="250"/>
</span>

Chameleon is a high-performance grammar-based generator for fuzzing.

__Pros:__
- Supports generating binary data in addition to text
- More than 2x faster than other generators
- Outputs C code instead of assembly, which means that it can utilize the
  full power of optimizing compilers
- Grammar syntax is a combination of protobuf and rust

__Cons:__
- Only for generation. Deserialization or grammar-based mutations are not supported

## Grammar
Similar to protobuf where an input is a sequence of messages, in Chameleon
an input is a sequence of `struct`s and a `struct` is just a container for variables.
We can define a `struct` like this:
```
struct ExampleStruct {
}
```
Inside the struct we have to put variables. The variable definition syntax is inspired by rusts
syntax so we first have the variable name, then a colon, then the type and then an assignment:
```
struct ExampleStruct {
    var_name: VarType = some_value;
}
```

### Variable Types
There are
- Numerical types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `char`
- String types: `string`, `bytes`
- Oneofs (just like in protobuf)
- Invocations of other structs

### Numerical types
Numerical types may or may not have an assignment.
Without an assignment like this
```
struct NumericalExample {
    numerical: u64;
}
```
any 64-bit value can be generated for this variable.
We can restrict the possible values for a numerical by specifying a numberset:
```
struct NumericalExample {
    numerical: u64 = 1,2,3..9,10;
}
```
In this numberset we have integers and ranges separated by commas.
Note that in a range `X..Y` the bounds are inclusive so the possible values
for the above variable are: { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }.
We can also specify negative numbers when the type is signed:
```
struct NumericalExample {
    numerical: i8 = -128..-1;
}
```
And we can choose from different number formats:
```
struct UnsignedFormats {
    decimal: u32 = 1,   10,     100;
    hex:     u32 = 0x1, 0xA,    0x64;
    oct:     u32 = 0o1, 0o12,   0o144;
    bin:     u32 = 0b1, 0b1010, 0b1100100;
}
```
If a non-decimal format is used for a signed integer it is interpreted
as a two's complement representation of the integer. Thus the variable `x` below gets assigned the value -1 in hex.
```
struct TwosComplement {
    x: i16 = 0xFFFF, -1; /* idempotent */
}
```
Finally with the `char` type we can also use character constants in numbersets:
```
struct HexChars {
    hex_chars: char = 'A'..'F', 'a'..'f', '0'..'9';
}
```

### String types
String types support two different kinds of assignments: a numberset and a literal.
Literals are constant strings enclosed in `"` and pretty straight-forward:
```
struct StringExample {
    hello: string = "Hello World!";
}
```
Numbersets tell the generator that the contents of the string will be random but the length
of the string is determined by the numberset.
The following variable will generate 10 to 20 random characters:
```
struct StringExample {
    random_chars: string = 10..20;
}
```
Note that if the type is `string` only valid ASCII characters will be generated.
If you want to use the full byte range 0-255 use a `bytes` type instead:
```
struct BytesExample {
    random_bytes: bytes = 10..20;
}
```

### Oneofs
Oneofs work just like in protobuf. It's a container with some variables and the generator picks exactly one of these variables:
```
struct OneofExample {
    hellos: oneof {
        _: string = "Hello World";
        _: string = "Hello there";
    };
}
```
Note the use of the empty variable name `_` here since we do not care about the names of the individual oneof-variants.

### Struct Invocations
Once you have defined a struct you can reference it in other structs, just like messages in protobuf.
```
struct A {
    /* ... */
}

struct B {
    invoke_a: A;
}
```

### Anonymous Structs
Often times it can become tedious to create external structs whenever you want to group some variables together.
For this reason Chameleon supports anonymous structs that can be used inline like this:
```
struct AnonymousExample {
    inline_group: struct {
        /* variables ... */
    };
}
```

### Variable Flags
protobuf supports marking variables as optional or repeated so we have the same flags here too.
They come directly before the variable name like this:
```
struct FlagsExample {
    optional     optional_var: string = "asdf";
    repeats 1..8 repeated_var: char   = '!';
}
```
In Chameleon there exists no unrestricted repeated flag. Instead you have to use the `repeats` flag which
takes a numberset as an argument that specifies the limits on how often a variable can be repeated.
In the example above at least one and at most 8 exclamation marks will be generated.

Flags can also be applied to oneofs and anonymous structs.

### Options
Options control the input generation of the generator and can be set either globally before any structs or locally in blocks enclosed by `{` and `}`.
Local options override global options for the scope of the current block and all child blocks.
The syntax for options is:
```
option <name> = <value>;
```

Currently there are three options:
- `endianness`: Sets the endianness of numericals (`native`, `little`, `big`, default: `native`)
- `scheduling`: Determines the strategy how to select variables from a oneof (`round-robin`, `random`, default: `round-robin`)
- `depth`: After `depth` items on the "call stack" the generator resorts to minimal expansion of all variables. Optionals are skipped,
  the minimum number of repetitions is chosen, etc. This helps in controlling the length of the generated input. Possible values:
  `unlimited` (default) or any number > 0.

Let's take the following struct for example
```
option endianness = little;
option depth = 8;

struct OptionsDemo {
    option endianness = big;
    
    numerical: u16 = 1;
    vars: oneof {
        option scheduling = random;
        
        var1: u16 = 2;
        var2: u16 = 3;
        /* ... */
    };
}
```
At the beginning we globally change two options. We set the endianness from `native` to `little` and set a `depth` of 8
instead of `unlimited`. These options now apply to every block that does not override these values. However, in our case the
endianness directly gets overriden and set to `big` in `OptionsDemo`. This means that `numerical` now becomes bigs endian (`00 01`)
just as `var1` and `var2`, since they are in child blocks of `OptionsDemo`.      
Lastly, the scheduling of `vars` get overriden with `random`  which means that for this block the generator randomly chooses which
variable to expand instead of selecting them in the order they were specified.

### Entrypoint
There must exist a struct with name `Root` which is the starting point for the generator.

## Usage
```
USAGE:
    chameleon [OPTIONS] <GRAMMAR>
```

### Actions
- `-o <OUTFILE>`: Generate a .c file that implements a generator for the specified grammar
- `--bench`: Convenience function that launches a benchmarking program for the specified grammar

### Options
- `--forbid-cycles`: Forbid cycles between structs
- `--prefix <PREFIX>`: Adds the prefix to all functions of the generators C API
- `--print-stats`: If the grammar does not contain cycles print some statistics

### Grammar
Grammars shall be stored with the `.chm` extension.     
Have at look at some [example grammars](./grammars) to get started.

## API
Once you have obtained a .c file you have access to the functions
- `void seed(size_t initial_seed)`: Seeds the internal PRNG
- `size_t generate(unsigned char* buf, size_t len)`: Construct an input and write 
  into the buffer specified by `buf` and `len` and return how many bytes were written

You can use the macros
- `MULTITHREADING`: Define this to mark every global variable as thread-local to make the generator thread-safe (off by default)
- `SEED=<N>`: Compile-time seed that is used when `seed()` is not called
- `DISABLE_rand`: Don't use the internal helper method `uint64_t rand()`. Can be used to provide a custom PRNG implementation.
- `DISABLE_random_buffer`: Don't use the internal helper method `void random_buffer (unsigned char* buf, uint32_t len, uint64_t mask)` that fills a given buffer
   with random data. Can be used to provide a custom implementation of the function.

## Evaluation
As a baseline fuzzer for comparison we chose [fzero_fuzzer](https://github.com/gamozolabs/fzero_fuzzer) since it
seemed to be the fastest generator freely available on Github at the time of writing this.    
We compared the JSON grammars [json.json](https://github.com/gamozolabs/fzero_fuzzer/blob/712a566d7acf7db6266417de2b2f9a06b5ca94e1/json.json) and [json.chm](grammars/json.chm) and recorded
- the throughput in MiB/s for different depths
- the sizes of the generated outputs for different depths

The comparisons were performed on an `Intel(R) Core(TM) i5-10210U CPU @ 1.60GHz`.   
You can find the setups in [./evaluation/](./evaluation).

### Throughput
fzero_fuzzer:

| depth | MiB/s |
| ---   | ---   |
| 8 | 72 |
| 16 | 76 |
| 32 | 77 |
| 64 | 75 |
| 128 | 75 |
| 256 | 72 |

Chameleon:

| depth | MiB/s |
| ---   | ---   |
| 8 | 160 |
| 16 | 170 |
| 32 | 160 |
| 64 | 170 |
| 128 | 170 |
| 256 | 170 |

### Output
fzero_fuzzer:    

| depth | samples | min. size | median size | avg. size | max. size |
| ---   | ---     | ---       | ---         | ---       | --- |
|  8   |  1048577 | 1 | 5 | 5    | 12 |
|  16  |  1048577 | 1 | 6 | 8    | 63 |
|  32  |  1048577 | 1 | 6 | 13   | 745 |
|  64  |  1048577 | 1 | 6 | 32   | 6820 |
|  128 |  1048577 | 1 | 6 | 153  | 50965 |
|  256 |  1048577 | 1 | 6 | 2915 | 1235896 |

for depths >= 512 fzero_fuzzer took too long (> 10 min.) to complete

Chameleon:

| depth | samples | min. size | median size | avg. size | max. size |
| ---   | ---     | ---       | ---         | ---       | --- |
| 8 | 1048577 | 1 | 9 | 28 | 390 |
| 16 | 1048577 | 1 | 10 | 65 | 2274 |
| 32 | 1048577 | 1 | 10 | 148 | 10821 |
| 64 | 1048577 | 1 | 9 | 310 | 52985 |
| 128 | 1048577 | 1 | 10 | 648 | 179996 |
| 256 | 1048577 | 1 | 10 | 1317 | 864252 |
| 512 | 1048577 | 1 | 10 | 2659 | 3551445 |
| 1024 | 1048577 | 1 | 10 | 5195 | 10560628 |
| 2048 | 1048577 | 1 | 10 | 10720 | 40372911 |
| 4096 | 1048577 | 1 | 10 | 21064 | 140085258 |
| 8192 | 1048577 | 1 | 10 | 43576 | 620817946 |

for depths >= 16384 Chameleon took too long (> 10 min.) to complete
