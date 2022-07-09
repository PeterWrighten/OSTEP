#include <stdio.h>
#include <unistd.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <string.h>
    
int main(void) {
    int i, pid, p[2], status;
    char buffer[20];

    pipe(p);
    pid = fork();
    if(pid == 0) {
        close(p[1]);
        if((i = read(p[0], buffer, 20)) == -1) {
            printf("child failed\n");
            exit(1);
        }
        printf("%s OK! I received your message.\n", buffer);
        exit(0);
    }
    close(p[0]);
    write(p[1], buffer, 20);
    wait(&status);
}
