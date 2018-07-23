#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

int main(int argc, char *argv[]) {
    if(argc > 2 || (argc > 1 && strcmp(argv[1], "-h") == 0)) {
        printf("usage: %s [ us ]\n", argv[0]);
        printf("\t'Narrow Pipe' copies chars from stdin to stdout, \n");
        printf("\tsleeping [ ns ] nanoseconds in between each one.\n");
        exit(0);
    }

    int sleeptime = 10000;
    if(argc == 2) {
        sleeptime = atoi(argv[1]);
    }

    int c;
    while(c = getchar(), c != -1) {
        putchar(c);
        fflush(stdout);
        usleep(sleeptime);
    }
}

