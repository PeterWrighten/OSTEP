#include <sys/types.h>
#include <unistd.h>
#include <stdio.h>

int main(void) {
    int pid;
    pid = fork();
    if(pid < 0) 
        printf("failed\n");
    else
        printf("I am a child\n");

    return 0;
}