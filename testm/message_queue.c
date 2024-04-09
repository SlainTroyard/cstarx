
#include <stdio.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>
#include <unistd.h>
#include <string.h>
#include <sys/wait.h>
#include <sys/msg.h>
 
#define MY_TYPE  9527
 
int main(void)
{
	int msgid;
	pid_t pid;
 
	struct msgbuf
	{
		long mtype;
		char mtext[100];
		int number;
	};
 
	struct msgbuf buff;
 
	msgid = msgget(IPC_PRIVATE, 0666 | IPC_EXCL); /* 不晓得为什么必须加上0666才可以*/
 
	if (msgid == -1) {
	    perror("msgget");
	    return -1;
	}
 
	pid = fork();
 
	if(pid > 0)
	{
	    sleep(1);
 
		buff.mtype = MY_TYPE;
		printf("Please enter a string you want to send:\n");
		gets(buff.mtext);
		printf("Please enter a nubmer you want to send:\n");
		scanf("%d", &buff.number);
 
		msgsnd(msgid, &buff, sizeof(buff) - sizeof(buff.mtype), 0);
 
		waitpid(pid, NULL, 0);
	}
	else if(pid == 0)
	{
		printf("Child process is waiting for msg:\n");
		msgrcv(msgid, &buff, sizeof(buff) - sizeof(buff.mtype), MY_TYPE, 0);
		printf("Child process read from msg: %s, %d\n", buff.mtext, buff.number);
	}
	else
		perror("fork");
 
	return 0;
}