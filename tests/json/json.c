
/************************************
     Auto-generated by Chameleon

              Parameters
             ~~~~~~~~~~~~
  Grammar: ../../grammars/json.chm
  Allow cycles: true
 ************************************/

#include <stddef.h>
#include <stdint.h>
#include <endian.h>

#define UNLIKELY(x) __builtin_expect(!!(x), 0)
#define LIKELY(x) __builtin_expect(!!(x), 1)

#ifndef __clang__
#define __builtin_memcpy_inline __builtin_memcpy
#endif

// Mark globals as thread local only if we are doing multithreading
#ifdef MULTITHREADING
#define THREAD_LOCAL __thread
#else
#define THREAD_LOCAL
#endif

// Define the compile-time seed
#ifndef SEED
#define SEED 0x35c6be9ba2548264
#endif

// Define endianness helper functions
#define LITTLE_ENDIAN_16(x) htole16((uint16_t) (x))
#define BIG_ENDIAN_16(x)    htobe16((uint16_t) (x))
#define LITTLE_ENDIAN_32(x) htole32((uint32_t) (x))
#define BIG_ENDIAN_32(x)    htobe32((uint32_t) (x))
#define LITTLE_ENDIAN_64(x) htole64((uint64_t) (x))
#define BIG_ENDIAN_64(x)    htobe64((uint64_t) (x))

// RNG: xorshift64
static THREAD_LOCAL uint64_t rand_state = SEED;

#ifndef DISABLE_rand
static uint64_t rand() {
    uint64_t x = rand_state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    return rand_state = x;
}
#else
uint64_t rand();
#endif

#ifndef DISABLE_seed
void seed(size_t s) {
    if (s) {
        rand_state = (uint64_t) s;
    } else {
        rand_state = SEED;
    }
}
#else
void seed(size_t);
#endif

// Helper method that writes random data into a buffer
#define MASK_BYTES 0xFFFFFFFFFFFFFFFFUL
#define MASK_STRING 0x7F7F7F7F7F7F7F7FUL
#ifndef DISABLE_random_buffer
static void random_buffer (unsigned char* buf, uint32_t len, uint64_t mask) {
    while (len >= 8) {
        *(uint64_t*)buf = rand() & mask;
        buf += 8; len -= 8;
    }
    
    while (len >= 4) {
        *(uint32_t*)buf = (uint32_t) (rand() & mask);
        buf += 4; len -= 4;
    }
    
    while (len >= 2) {
        *(uint16_t*)buf = (uint16_t) (rand() & mask);
        buf += 2; len -= 2;
    }
    
    while (len >= 1) {
        *buf = (unsigned char) (rand() & mask);
        buf += 1; len -= 1;
    }
}
#else
void random_buffer (unsigned char* buf, uint32_t len, uint64_t mask);
#endif

// Strings from grammar
static const unsigned char string_3946110457539242724[5] = {0x66, 0x61, 0x6c, 0x73, 0x65};
static const unsigned char string_6895804349017018507[4] = {0x6e, 0x75, 0x6c, 0x6c};
static const unsigned char string_8179814853952207734[4] = {0x74, 0x72, 0x75, 0x65};

// Numbersets from grammar
static uint8_t numberset_304032344720464445() {
    static THREAD_LOCAL uint64_t numberset_cursor = 0;
    uint64_t numberset_selector = numberset_cursor++ % 3;
    switch(numberset_selector) {
        case 0: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 2U) + 32U;
        }
        case 1: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 57U) + 35U;
        }
        case 2: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 35U) + 93U;
        }
        default: {
            __builtin_unreachable();
        }
    }
}
static uint8_t numberset_535708458095750977() {
            return 45U;
}
static uint8_t numberset_570823944604184829() {
            return 125U;
}
static uint8_t numberset_831432947624321420() {
            return 92U;
}
static uint32_t numberset_2742457571863202324() {
            return 4U;
}
static uint8_t numberset_2974950330330226270() {
            return 91U;
}
static uint8_t numberset_3037289215760968923() {
    static THREAD_LOCAL uint64_t numberset_cursor = 0;
    uint64_t numberset_selector = numberset_cursor++ % 2;
    switch(numberset_selector) {
        case 0: {
            return 43U;
        }
        case 1: {
            return 45U;
        }
        default: {
            __builtin_unreachable();
        }
    }
}
static uint8_t numberset_3647269789286519387() {
            return 34U;
}
static uint8_t numberset_4625647195457032579() {
            return 44U;
}
static uint8_t numberset_6484256901419378940() {
    static THREAD_LOCAL uint64_t numberset_cursor = 0;
    uint64_t numberset_selector = numberset_cursor++ % 2;
    switch(numberset_selector) {
        case 0: {
            return 69U;
        }
        case 1: {
            return 101U;
        }
        default: {
            __builtin_unreachable();
        }
    }
}
static uint8_t numberset_6579617425334308193() {
    static THREAD_LOCAL uint64_t numberset_cursor = 0;
    uint64_t numberset_selector = numberset_cursor++ % 3;
    switch(numberset_selector) {
        case 0: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 10U) + 48U;
        }
        case 1: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 6U) + 65U;
        }
        case 2: {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 6U) + 97U;
        }
        default: {
            __builtin_unreachable();
        }
    }
}
static uint32_t numberset_8885837958780172474() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint32_t) range_selector) % 9U) + 0U;
}
static uint8_t numberset_10366812475632120030() {
            return 123U;
}
static uint8_t numberset_11185474807884195476() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 10U) + 48U;
}
static uint32_t numberset_11507316992910836708() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint32_t) range_selector) % 20U) + 1U;
}
static uint32_t numberset_12420855493989175532() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint32_t) range_selector) % 17U) + 0U;
}
static uint8_t numberset_13374493992030014863() {
            return 46U;
}
static uint8_t numberset_15518408013711948167() {
            return 93U;
}
static uint8_t numberset_16481366060854711735() {
            return 58U;
}
static uint8_t numberset_16687042391787592567() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint8_t) range_selector) % 9U) + 49U;
}
static uint8_t numberset_16691176133774852529() {
            return 48U;
}
static uint32_t numberset_16876133892018511142() {
            static THREAD_LOCAL uint64_t range_cursor = 0;
            uint64_t range_selector = range_cursor++;
            return (((uint32_t) range_selector) % 20U) + 0U;
}
static uint8_t numberset_17129939822146248260() {
    static THREAD_LOCAL uint64_t numberset_cursor = 0;
    uint64_t numberset_selector = numberset_cursor++ % 8;
    switch(numberset_selector) {
        case 0: {
            return 34U;
        }
        case 1: {
            return 47U;
        }
        case 2: {
            return 92U;
        }
        case 3: {
            return 98U;
        }
        case 4: {
            return 102U;
        }
        case 5: {
            return 110U;
        }
        case 6: {
            return 114U;
        }
        case 7: {
            return 116U;
        }
        default: {
            __builtin_unreachable();
        }
    }
}
static uint8_t numberset_18117484783764985230() {
            return 117U;
}

// Forward declarations of containers
static size_t container_0(unsigned char*, size_t);
static size_t container_1(unsigned char*, size_t);
static size_t container_2(unsigned char*, size_t);
static size_t container_3(unsigned char*, size_t);
static size_t container_4(unsigned char*, size_t);
static size_t container_5(unsigned char*, size_t);
static size_t container_6(unsigned char*, size_t);
static size_t container_7(unsigned char*, size_t);
static size_t container_8(unsigned char*, size_t);
static size_t container_9(unsigned char*, size_t);
static size_t container_10(unsigned char*, size_t);
static size_t container_11(unsigned char*, size_t);
static size_t container_12(unsigned char*, size_t);
static size_t container_13(unsigned char*, size_t);
static size_t container_14(unsigned char*, size_t);
static size_t container_15(unsigned char*, size_t);
static size_t container_16(unsigned char*, size_t);
static size_t container_17(unsigned char*, size_t);
static size_t container_18(unsigned char*, size_t);
static size_t container_19(unsigned char*, size_t);
static size_t container_20(unsigned char*, size_t);
static size_t container_21(unsigned char*, size_t);
static size_t container_22(unsigned char*, size_t);
static size_t container_23(unsigned char*, size_t);

// Definition of containers
static size_t container_0(unsigned char* buf, size_t len) {
    // This container is struct Char
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_304032344720464445();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_1(unsigned char* buf, size_t len) {
    // This container is struct HexDigit
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_6579617425334308193();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_2(unsigned char* buf, size_t len) {
    // This container is struct String
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_3647269789286519387();
        buf += 1; len -= 1;
    }
    {
        uint32_t repeats_i = numberset_12420855493989175532();
        while (repeats_i--) {
        size_t container_len = container_3(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_3647269789286519387();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_3(unsigned char* buf, size_t len) {
    size_t original_len = len;
    static THREAD_LOCAL uint64_t oneof_cursor = 0;
    uint64_t oneof_selector = oneof_cursor++ % 2;
    switch(oneof_selector) {
        case 0: {
        size_t container_len = container_0(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        case 1: {
        size_t container_len = container_4(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        default: {
            __builtin_unreachable();
        }
    }
    return original_len - len;
}
static size_t container_4(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 13 column 9
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_831432947624321420();
        buf += 1; len -= 1;
    }
    {
        size_t container_len = container_5(buf, len);
        buf += container_len; len -= container_len;
    }
  container_end:
    return original_len - len;
}
static size_t container_5(unsigned char* buf, size_t len) {
    size_t original_len = len;
    static THREAD_LOCAL uint64_t oneof_cursor = 0;
    uint64_t oneof_selector = oneof_cursor++ % 2;
    switch(oneof_selector) {
        case 0: {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_17129939822146248260();
        buf += 1; len -= 1;
        break;
        }
        case 1: {
        size_t container_len = container_6(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        default: {
            __builtin_unreachable();
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_6(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 17 column 17
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_18117484783764985230();
        buf += 1; len -= 1;
    }
    {
        uint32_t repeats_i = numberset_2742457571863202324();
        while (repeats_i--) {
        size_t container_len = container_1(buf, len);
        buf += container_len; len -= container_len;
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_7(unsigned char* buf, size_t len) {
    // This container is struct Member
    size_t original_len = len;
    {
        size_t container_len = container_2(buf, len);
        buf += container_len; len -= container_len;
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_16481366060854711735();
        buf += 1; len -= 1;
    }
    {
        size_t container_len = container_20(buf, len);
        buf += container_len; len -= container_len;
    }
  container_end:
    return original_len - len;
}
static size_t container_8(unsigned char* buf, size_t len) {
    // This container is struct Object
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_10366812475632120030();
        buf += 1; len -= 1;
    }
    {
        static THREAD_LOCAL uint64_t optional_counter = 0;
        if (optional_counter++ & 1) {
        size_t container_len = container_9(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_570823944604184829();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_9(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 35 column 5
    size_t original_len = len;
    {
        uint32_t repeats_i = numberset_8885837958780172474();
        while (repeats_i--) {
        size_t container_len = container_10(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        size_t container_len = container_7(buf, len);
        buf += container_len; len -= container_len;
    }
    return original_len - len;
}
static size_t container_10(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 36 column 9
    size_t original_len = len;
    {
        size_t container_len = container_7(buf, len);
        buf += container_len; len -= container_len;
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_4625647195457032579();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_11(unsigned char* buf, size_t len) {
    // This container is struct Array
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_2974950330330226270();
        buf += 1; len -= 1;
    }
    {
        static THREAD_LOCAL uint64_t optional_counter = 0;
        if (optional_counter++ & 1) {
        size_t container_len = container_12(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_15518408013711948167();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_12(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 47 column 5
    size_t original_len = len;
    {
        uint32_t repeats_i = numberset_8885837958780172474();
        while (repeats_i--) {
        size_t container_len = container_13(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        size_t container_len = container_20(buf, len);
        buf += container_len; len -= container_len;
    }
    return original_len - len;
}
static size_t container_13(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 48 column 9
    size_t original_len = len;
    {
        size_t container_len = container_20(buf, len);
        buf += container_len; len -= container_len;
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_4625647195457032579();
        buf += 1; len -= 1;
    }
  container_end:
    return original_len - len;
}
static size_t container_14(unsigned char* buf, size_t len) {
    // This container is struct Digits
    size_t original_len = len;
    {
        size_t container_len = container_15(buf, len);
        buf += container_len; len -= container_len;
    }
    return original_len - len;
}
static size_t container_15(unsigned char* buf, size_t len) {
    size_t original_len = len;
    static THREAD_LOCAL uint64_t oneof_cursor = 0;
    uint64_t oneof_selector = oneof_cursor++ % 2;
    switch(oneof_selector) {
        case 0: {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_16691176133774852529();
        buf += 1; len -= 1;
        break;
        }
        case 1: {
        size_t container_len = container_16(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        default: {
            __builtin_unreachable();
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_16(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 60 column 9
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_16687042391787592567();
        buf += 1; len -= 1;
    }
    {
        uint32_t repeats_i = numberset_16876133892018511142();
        while (repeats_i--) {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_11185474807884195476();
        buf += 1; len -= 1;
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_17(unsigned char* buf, size_t len) {
    // This container is struct Number
    size_t original_len = len;
    {
        static THREAD_LOCAL uint64_t optional_counter = 0;
        if (optional_counter++ & 1) {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_535708458095750977();
        buf += 1; len -= 1;
        }
    }
    {
        size_t container_len = container_14(buf, len);
        buf += container_len; len -= container_len;
    }
    {
        static THREAD_LOCAL uint64_t optional_counter = 0;
        if (optional_counter++ & 1) {
        size_t container_len = container_18(buf, len);
        buf += container_len; len -= container_len;
        }
    }
    {
        static THREAD_LOCAL uint64_t optional_counter = 0;
        if (optional_counter++ & 1) {
        size_t container_len = container_19(buf, len);
        buf += container_len; len -= container_len;
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_18(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 70 column 5
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_13374493992030014863();
        buf += 1; len -= 1;
    }
    {
        uint32_t repeats_i = numberset_11507316992910836708();
        while (repeats_i--) {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_11185474807884195476();
        buf += 1; len -= 1;
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_19(unsigned char* buf, size_t len) {
    // This container is the anonymous struct in line 74 column 5
    size_t original_len = len;
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_6484256901419378940();
        buf += 1; len -= 1;
    }
    {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_3037289215760968923();
        buf += 1; len -= 1;
    }
    {
        uint32_t repeats_i = numberset_11507316992910836708();
        while (repeats_i--) {
        if (UNLIKELY(len < 1)) {
            goto container_end;
        }
        *buf = (unsigned char) numberset_11185474807884195476();
        buf += 1; len -= 1;
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_20(unsigned char* buf, size_t len) {
    // This container is struct Value
    size_t original_len = len;
    {
        size_t container_len = container_21(buf, len);
        buf += container_len; len -= container_len;
    }
    return original_len - len;
}
static size_t container_21(unsigned char* buf, size_t len) {
    size_t original_len = len;
    uint64_t oneof_selector = rand() % 5;
    switch(oneof_selector) {
        case 0: {
        size_t container_len = container_8(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        case 1: {
        size_t container_len = container_11(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        case 2: {
        size_t container_len = container_2(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        case 3: {
        size_t container_len = container_17(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        case 4: {
        size_t container_len = container_22(buf, len);
        buf += container_len; len -= container_len;
        break;
        }
        default: {
            __builtin_unreachable();
        }
    }
    return original_len - len;
}
static size_t container_22(unsigned char* buf, size_t len) {
    size_t original_len = len;
    static THREAD_LOCAL uint64_t oneof_cursor = 0;
    uint64_t oneof_selector = oneof_cursor++ % 3;
    switch(oneof_selector) {
        case 0: {
        if (UNLIKELY(len < sizeof(string_8179814853952207734))) {
            goto container_end;
        }
        __builtin_memcpy_inline(buf, string_8179814853952207734, sizeof(string_8179814853952207734));
        buf += sizeof(string_8179814853952207734); len -= sizeof(string_8179814853952207734);
        break;
        }
        case 1: {
        if (UNLIKELY(len < sizeof(string_3946110457539242724))) {
            goto container_end;
        }
        __builtin_memcpy_inline(buf, string_3946110457539242724, sizeof(string_3946110457539242724));
        buf += sizeof(string_3946110457539242724); len -= sizeof(string_3946110457539242724);
        break;
        }
        case 2: {
        if (UNLIKELY(len < sizeof(string_6895804349017018507))) {
            goto container_end;
        }
        __builtin_memcpy_inline(buf, string_6895804349017018507, sizeof(string_6895804349017018507));
        buf += sizeof(string_6895804349017018507); len -= sizeof(string_6895804349017018507);
        break;
        }
        default: {
            __builtin_unreachable();
        }
    }
  container_end:
    return original_len - len;
}
static size_t container_23(unsigned char* buf, size_t len) {
    // This container is struct Root
    size_t original_len = len;
    {
        size_t container_len = container_20(buf, len);
        buf += container_len; len -= container_len;
    }
    return original_len - len;
}

// Entrypoint for the generator
size_t generate(unsigned char* buf, size_t len) {
    if (UNLIKELY(!buf || !len)) {
        return 0;
    }
    
    return container_23(buf, len);
}
