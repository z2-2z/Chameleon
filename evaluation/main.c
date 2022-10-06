// gcc -Wall -Wextra -Wpedantic -Wno-unused-function -Wno-unused-parameter -Werror -O3 -flto -o generator main.c generator.c

#include <stdio.h>
#include <stdlib.h>

#include "generator.c.h"

unsigned char buffer[1024 * 1024 * 1024] = {0};

int compare (const void* a, const void* b) {
    size_t left = *(size_t*)a;
    size_t right = *(size_t*)b;
    
    if (left < right) {
        return -1;
    } else if (left == right) {
        return 0;
    } else {
        return 1;
    }
}

int main (void) {
    size_t samples = 1024 * 1024 + 1;
    size_t* sizes = calloc(samples, sizeof(size_t));
    
    if (!sizes) {
        return 1;
    }
    
    size_t avg = 0;
    
    for (size_t i = 0; i < samples; ++i) {
        sizes[i] = generate(buffer, sizeof(buffer));
        avg += sizes[i];
    }
    
    qsort(sizes, samples, sizeof(size_t), compare);
    
    size_t min = sizes[0];
    size_t median = sizes[samples / 2];
    size_t max = sizes[samples - 1];
    avg /= samples;
    
    printf("depth: ? | samples: %lu | min: %lu | median: %lu | avg: %lu | max: %lu\n", samples, min, median, avg, max);
    
    return 0;
}
