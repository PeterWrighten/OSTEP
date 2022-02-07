#include <string.h>
#include <sys/wait.h>
#include <stdio.h>

void main(void) {
    int i, pid, p[2], status;
    char buffer[20];

    pipe(p);
    if(pid = fork() == 0) {
        close(p[1]);
        if((i = read(p[0], buffer, 20)) == -1) {
            printf("child failed\n");
            _exit(1);
        }
        printf("%s OK! I received your message.\n", buffer);
        _exit(0);
    }
    close(p[0]);
    write(p[1]);
    wait(&status);

}