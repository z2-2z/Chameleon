#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stddef.h>
#include <time.h>
#include "json.c.h"

int main (int argc, char** argv) {
    if (argc < 2) {
        printf("Usage: %s {gen | bench}\n", argv[0]);
        return 1;
    }
    
    size_t len = 1024 * 1024 * 1024;
    unsigned char* buf = malloc(len);
    
    if (!buf) {
        return 1;
    }
    
    if (strcmp(argv[1], "gen") == 0) {
        seed(time(NULL) + (long) argv);
        size_t gen_len = generate(buf, len);
        if (gen_len == 0) {
            return 1;
        }
        
        if (write(1, buf, gen_len) != (ssize_t) gen_len) {
            return 1;
        }
    } else if (strcmp(argv[1], "bench") == 0) {
        seed(time(NULL) + (long) argv);
        unsigned long generated = 0;
        struct timespec start;
        
        clock_gettime(CLOCK_MONOTONIC, &start);
        
        // Mimic fzero_fuzzers benchmarking
        for (unsigned long i = 1;; ++i) {
            generated += generate(buf, len);
            
            if ((i & 0xffff) == 0) {
                struct timespec now;
                clock_gettime(CLOCK_MONOTONIC, &now);
                
                double elapsed = (double) (now.tv_sec - start.tv_sec);
                double bytes_per_sec = (double)generated / elapsed;
                unsigned long avg_size = generated / i;
                
                printf("MiB/sec: %12.4lf | avg size: %lu\n", bytes_per_sec / 1024.0 / 1024.0, avg_size);
            }
        }
    } else {
        printf("do you are have stupid?\n");
    }
    
    return 0;
}
