# Chameleon (beta)

Chameleon is a grammar-based generator for fuzzing.

It takes a grammar as input, whose syntax resembles the
protobuf format and outputs C code that can be called
by other applications.
However in contrast to protobuf the generated code does
not parse any input into a predefined structure. It is
for generation only.

The C output is highly performance optimized and currently
outperforms other fast generators like [fzero_fuzzer](https://github.com/gamozolabs/fzero_fuzzer)
by a factor of at least 2x.

Futhermore Chameleon comes with different input generation strategies:
- deterministic enumeration of the input space (default, fastest)
- purely random generation (slower)

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
And when the type is unsigned we can choose from different number formats:
```
struct UnsignedFormats {
    decimal: u32 = 1,   10,     100;
    hex:     u32 = 0x1, 0xA,    0x64;
    oct:     u32 = 0o1, 0o12,   0o144;
    bin:     u32 = 0b1, 0b1010, 0b1100100;
}
```
Finally with the `char` type we can also use character constants in numbersets:
```
struct CharsExample {
    some_chars: char = 'A', 'B', 'C';
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
Options control the input generation of the generator and can be globally set at the beginning of the file or
locally in oneofs or structs.
Options supplied in structs or oneofs override the global or default options for the local scope.
The syntax is:
```
option <name> = <value>;
```

Currently there are two options:
- `endianness`: Sets the endianness of numericals (`native`, `little`, `big`, default: `native`)
- `scheduling`: Determines the strategy how to select numbers from a numberset and variables from a oneof (`round-robin`, `random`, default: `round-robin`)

Let's take the following struct for example
```
option endianness = big;

struct OptionsEffects {
    numerical: u16 = 1;
    vars: oneof {
        var1: u16 = 2;
        var2: u16 = 3;
        /* ... */
    };
}
```
Since the endianness was set to big the output of the generator will be `00 01` instead of `01 00` for variable `numerical`.
The scheduling has not been set so it defaults to `round-robin` which means that in the first iteration `var1` gets used,
in the second iteration `var2` gets used and so on.
Would the scheduling have been set to `random` then a random variant would be chosen by the generator.

Note that the scheduling also affects what numbers will be generated by a numberset.
If we have the numberset below
```
struct SchedulingComplexities {
    _: oneof {
        var1: u8 = 1,2,3;
        var2: u8 = 5,6,7;
    };
}
```
then with `round-robin` we get the outputs 1, 5, 2, 6, 3, 7 (breadth-first-search).
If it is more desired to randomly get numbers from a numberset we
can set the global scheduling to random and the local scheduling to round-robin:
```
option scheduling = random;

struct SchedulingComplexities {
    _: oneof {
        option scheduling = round-robin;
        var1: u8 = 1,2,3;
        var2: u8 = 5,6,7;
        /* ... */
    };
}
```
This works because local options never affect numbersets due to implementation details.
So in this case we will still get the desired order for the oneof: `var1`, `var2`, etc.
but the values of the variables will be randomly chosen.

### Entrypoint
There must exist a struct with name `Root` which is the starting point for the generator.

## Usage
TODO: table of options

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

## Examples
Have at look at some [example grammars](./grammars) to get started.

