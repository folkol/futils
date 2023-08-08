#include <stdio.h>
#include <stdlib.h>

#define BUF_SIZE 1024

int main(void) {
    long sum = 0;
    size_t num_read;
    uint32_t buf[BUF_SIZE];
    while(num_read = fread(buf, sizeof (uint32_t), BUF_SIZE, stdin), num_read > 0) {
        for(int i = 0; i < num_read; i++) {
            sum += buf[i];
        }
    }
    printf("%ld\n", sum);
}

